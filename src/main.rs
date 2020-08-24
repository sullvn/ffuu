use anyhow::anyhow;
use async_std::fs::{create_dir_all, read, read_dir, remove_dir_all, write};
use async_std::io;
use async_std::path::Path;
use async_std::prelude::StreamExt;
use async_std::task::spawn;
use futures::future;
use pulldown_cmark::{html, CowStr, Event, Parser};
use std::env;
use std::io::Write;
use std::ops::Range;
use std::process::{Command, Stdio};
use std::str;

mod lib;

use lib::{HTMLTag, HTMLTagKind};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).ok_or(anyhow!("Missing input directory path"))?;
    let output_path = args
        .get(2)
        .ok_or(anyhow!("Missing output directory path"))?;

    match remove_dir_all(output_path).await {
        Err(err) if err.kind() == io::ErrorKind::NotFound => Ok(()),
        r => r,
    }?;
    create_dir_all(output_path).await?;

    let mut tasks = Vec::new();
    let mut entries = read_dir(input_path).await?;
    while let Some(entry) = entries.next().await {
        match entry {
            Ok(entry) => {
                let handle = spawn(write_html(entry.path(), output_path.clone()));
                tasks.push(handle);
            }
            _ => {}
        };
    }

    future::join_all(tasks).await;
    Ok(())
}

async fn write_html<P: AsRef<Path>, Q: AsRef<Path>>(
    input_path: P,
    output_dir: Q,
) -> anyhow::Result<()> {
    let input_file_name = input_path
        .as_ref()
        .file_name()
        .ok_or(anyhow!("No filename: {:?}", input_path.as_ref()))?;
    let output_path = output_dir
        .as_ref()
        .join(input_file_name)
        .with_extension("html");

    let input_bytes = read(input_path).await?;
    let input_text: &str = str::from_utf8(&input_bytes)?;

    let mut html_output = String::new();
    {
        let (mut pieces, embed_requests) = do_parse(input_text);
        let embed_results: Vec<(anyhow::Result<String>, EmbedRequest)> = embed_requests
            .into_iter()
            .map(|er| (do_embed_request(&er), er))
            .collect();

        for (piece, er) in &embed_results {
            pieces[er.piece_index] = embed_result_piece(piece);
        }

        let embarkdown = EmbarkdownParser::new(pieces);
        html::push_html(&mut html_output, embarkdown);
    }

    // TODO: Get rid of buffering here
    write(output_path, html_output).await?;

    Ok(())
}

fn do_embed_request(request: &EmbedRequest) -> anyhow::Result<String> {
    let source = exec_embed(request)?;
    let text = String::from_utf8(source)?;

    Ok(text)
}

fn embed_result_piece<'a>(result: &'a anyhow::Result<String>) -> Piece<'a> {
    match result {
        Ok(embed_text) => Piece::EmbedResult(Parser::new(embed_text.trim())),
        Err(_) => Piece::EmbedError,
    }
}

#[derive(Debug)]
enum EmbedParsing<'a> {
    None,
    Start {
        executable: &'a str,
        args: Option<&'a str>,
    },
    Partial {
        executable: &'a str,
        args: Option<&'a str>,
        range: Range<usize>,
    },
}

#[derive(Debug)]
struct EmbedRequest<'a> {
    executable: &'a str,
    input: &'a str,
    args: Option<&'a str>,
    piece_index: usize,
}

enum Piece<'a> {
    Markdown(Event<'a>),
    EmbedPending,
    EmbedResult(Parser<'a>),
    EmbedError,
}

struct EmbarkdownParser<'a> {
    pieces: std::vec::IntoIter<Piece<'a>>,
    embed_result_parser: Option<Parser<'a>>,
}

impl<'a> EmbarkdownParser<'a> {
    fn new(pieces: Vec<Piece<'a>>) -> EmbarkdownParser<'a> {
        EmbarkdownParser {
            pieces: pieces.into_iter(),
            embed_result_parser: None,
        }
    }

    fn next_embed_result(&mut self) -> Option<Event<'a>> {
        match &mut self.embed_result_parser {
            Some(parser) => parser.next(),
            None => None,
        }
    }
}

impl<'a> Iterator for EmbarkdownParser<'a> {
    type Item = Event<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(er) = self.next_embed_result() {
            return Some(er);
        }

        let p = self.pieces.next()?;
        match p {
            Piece::Markdown(m) => Some(m),
            Piece::EmbedResult(parser) => {
                self.embed_result_parser = Some(parser);
                self.next()
            }
            Piece::EmbedPending | Piece::EmbedError => panic!("Parsing error"),
        }
    }
}

fn exec_embed(request: &EmbedRequest) -> anyhow::Result<Vec<u8>> {
    let EmbedRequest {
        executable,
        args: maybe_args,
        input,
        ..
    } = request;
    let split_args = maybe_args.unwrap_or("").split(' ');
    let mut child = Command::new(executable)
        .args(split_args)
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .spawn()?;

    child
        .stdin
        .as_mut()
        .ok_or(anyhow!("Can't borrow stdin as mutable"))?
        .write_all(input.as_bytes())?;
    let output = child.wait_with_output()?;

    Ok(output.stdout)
}

fn do_parse<'a>(text: &'a str) -> (Vec<Piece<'a>>, Vec<EmbedRequest<'a>>) {
    let mut depth: isize = 0;
    let mut embed_request = EmbedParsing::None;
    let mut pieces: Vec<Piece> = Vec::new();
    let mut embed_requests: Vec<EmbedRequest> = Vec::new();

    let mut md_offset_events = Parser::new(text).into_offset_iter();
    while let Some((event, range)) = md_offset_events.next() {
        let html_tag = if let Event::Html(CowStr::Borrowed(tag)) = event {
            HTMLTag::parse(&tag).map(|(_input, tag)| tag).ok()
        } else {
            None
        };

        match (depth, &html_tag, &mut embed_request) {
            //
            // Starting an embed
            //
            (
                0,
                Some(HTMLTag {
                    kind: HTMLTagKind::Open,
                    name,
                    attributes,
                }),
                EmbedParsing::None,
            ) => {
                let args = attributes
                    .into_iter()
                    .find(|(attr_name, _)| attr_name == &"args")
                    .map(|(_, args_value)| *args_value);
                embed_request = EmbedParsing::Start {
                    executable: name,
                    args,
                };
            }
            //
            // Ending an embed
            //
            (
                1,
                Some(HTMLTag {
                    kind: HTMLTagKind::Close,
                    ..
                }),
                EmbedParsing::Partial {
                    executable,
                    args,
                    range: Range { start, end },
                },
            ) => {
                embed_requests.push(EmbedRequest {
                    executable,
                    args: *args,
                    input: &text[*start..*end],
                    piece_index: pieces.len(),
                });
                pieces.push(Piece::EmbedPending);
                embed_request = EmbedParsing::None;
            }
            (_, _, EmbedParsing::Start { executable, args }) => {
                embed_request = EmbedParsing::Partial {
                    executable,
                    args: *args,
                    range,
                };
            }
            (
                _,
                _,
                EmbedParsing::Partial {
                    range: req_range, ..
                },
            ) => {
                req_range.end = range.end;
            }
            _ => {}
        };

        if html_tag.is_none() && depth < 1 {
            pieces.push(Piece::Markdown(event));
        }

        if let Some(HTMLTag { kind, .. }) = html_tag {
            depth += kind.depth_change();
        }
    }

    (pieces, embed_requests)
}
