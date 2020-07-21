use std::env;
use std::str;
use anyhow::anyhow;
use async_std::prelude::StreamExt;
use async_std::fs::{read, write, read_dir, remove_dir_all, create_dir_all};
use async_std::path::Path;
use async_std::task::spawn;
use async_std::io;
use futures::future;
use pulldown_cmark::{Parser, html, Event, CowStr};

#[async_std::main]
async fn main() -> anyhow::Result<()> {
    let args: Vec<String> = env::args().collect();
    let input_path = args.get(1).ok_or(anyhow!("Missing input directory path"))?;
    let output_path = args.get(2).ok_or(anyhow!("Missing output directory path"))?;

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
            },
            _ => {},
        };
    }

    future::join_all(tasks).await;
    Ok(())
}

async fn write_html<P: AsRef<Path>, Q: AsRef<Path>>(input_path: P, output_dir: Q) -> anyhow::Result<()> {
    let input_file_name = input_path.as_ref().file_name().ok_or(anyhow!("No filename: {:?}", input_path.as_ref()))?;
    let output_path = output_dir.as_ref().join(input_file_name).with_extension("html");

    let input_bytes = read(input_path).await?;
    let input_text: &str = str::from_utf8(&input_bytes)?;

    do_parse(input_text);

    let mut html_output = String::new();
    {
        let md_parser = Parser::new(input_text);
        html::push_html(&mut html_output, md_parser);
    }

    write(output_path, html_output).await?;

    Ok(())
}

fn do_parse(text: &str) {
    let mut md_parser = Parser::new(text);
    let mut depth: isize = 0;
    let mut curr_tag: Option<&str> = None;

    while let Some(event) = md_parser.next() {
        if let Event::Html(CowStr::Borrowed(tag)) = event {
            if let Some(HtmlTag { name, direction }) = parse_tag(&tag) {
                match (depth, &direction, curr_tag) {
                    (0,  TagDirection::Open, _) => {
                        curr_tag = Some(name);
                    },
                    (1, TagDirection::Close, Some(curr_tag_name)) => {
                        println!("{}", curr_tag_name);
                        curr_tag = None;
                    },
                    _ => {},
                };

                depth += direction.depth_change();
            }
        }
    }
}

#[derive(Eq, PartialEq, Debug)]
enum TagDirection {
    Open,
    Close,
}

impl TagDirection {
    fn depth_change(&self) -> isize {
        match self {
            TagDirection::Open => 1,
            TagDirection::Close => -1,
        }
    }
}

struct HtmlTag<'a> {
    name: &'a str,
    direction: TagDirection,
}


fn parse_tag<'a>(s: &'a str) -> Option<HtmlTag<'a>> {
    let without_prefix = s.trim().strip_prefix('<')?;
    let name_end = without_prefix.find(|c: char|
        c.is_whitespace() || c == '>')?;
    let name = without_prefix.get(0..name_end)?;

    let html_tag = match name.strip_prefix('/') {
        None => HtmlTag {
            name,
            direction: TagDirection::Open,
        },
        Some(name) => HtmlTag {
            name,
            direction: TagDirection::Close,
        },
    };

    Some(html_tag)
}