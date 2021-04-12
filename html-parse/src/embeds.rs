use crate::depth::WithDepthIterator;
use crate::{format_html, HTMLPart, HTMLTag, HTMLTagKind};
use std::borrow::Cow;

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HTMLEmbed<'a> {
    pub command: &'a str,
    pub input: Option<Cow<'a, str>>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum HTMLPartOrEmbed<'a> {
    Part(HTMLPart<'a>),
    Embed(HTMLEmbed<'a>),
}

impl<'a> From<&HTMLPart<'a>> for Option<HTMLEmbed<'a>> {
    fn from(part: &HTMLPart<'a>) -> Self {
        match part {
            HTMLPart::Tag(HTMLTag {
                name: "run",
                kind,
                attributes,
            }) if *kind == HTMLTagKind::Open || *kind == HTMLTagKind::Void => attributes
                .iter()
                .find(|(k, _)| *k == "command")
                .map(|(_, v)| *v)
                .flatten()
                .map(|command| HTMLEmbed {
                    command,
                    input: None,
                }),
            _ => None,
        }
    }
}

struct PendingHTMLEmbed<'a> {
    command: &'a str,
    depth: isize,
    input_parts: Vec<HTMLPart<'a>>,
}

#[allow(dead_code)]
pub fn parse_embeds<'a>(html_parts: Vec<HTMLPart<'a>>) -> Vec<HTMLPartOrEmbed<'a>> {
    let mut html_parts_or_embeds = Vec::new();
    let mut maybe_pending_embed: Option<PendingHTMLEmbed<'a>> = None;

    for (hp, depth) in html_parts.into_iter().with_depth() {
        let maybe_new_embed: Option<HTMLEmbed<'a>> = (&hp).into();
        let is_embed_end: bool = match (&maybe_pending_embed, &hp) {
            (
                Some(PendingHTMLEmbed {
                    depth: pe_depth, ..
                }),
                HTMLPart::Tag(HTMLTag {
                    name: "run",
                    kind: HTMLTagKind::Close,
                    ..
                }),
            ) if depth <= *pe_depth => true,
            _ => false,
        };

        match (&mut maybe_pending_embed, maybe_new_embed, &hp, is_embed_end) {
            (
                None,
                Some(new_embed),
                &HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Void,
                    ..
                }),
                false,
            ) => html_parts_or_embeds.push(HTMLPartOrEmbed::Embed(HTMLEmbed {
                command: new_embed.command,
                input: None,
            })),
            (
                None,
                Some(new_embed),
                &HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Open,
                    ..
                }),
                false,
            ) => {
                maybe_pending_embed = Some(PendingHTMLEmbed {
                    command: new_embed.command,
                    depth: 0,
                    input_parts: Vec::new(),
                })
            }
            (Some(pending_embed), _, _, false) => pending_embed.input_parts.push(hp),
            (Some(finished_embed), None, _, true) => {
                let input_formatted = format_html(&finished_embed.input_parts);
                html_parts_or_embeds.push(HTMLPartOrEmbed::Embed(HTMLEmbed {
                    command: finished_embed.command,
                    input: Some(input_formatted.into()),
                }));
                maybe_pending_embed = None;
            }
            _ => html_parts_or_embeds.push(HTMLPartOrEmbed::Part(hp)),
        };
    }

    html_parts_or_embeds
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_no_embed() {
        assert_eq!(
            parse_embeds(vec![
                HTMLPart::Tag(HTMLTag {
                    name: "p",
                    kind: HTMLTagKind::Open,
                    attributes: Vec::new(),
                }),
                HTMLPart::Text("Paragraph text.".into()),
                HTMLPart::Tag(HTMLTag {
                    name: "p",
                    kind: HTMLTagKind::Close,
                    attributes: Vec::new(),
                }),
            ]),
            vec![
                HTMLPartOrEmbed::Part(HTMLPart::Tag(HTMLTag {
                    name: "p",
                    kind: HTMLTagKind::Open,
                    attributes: Vec::new(),
                })),
                HTMLPartOrEmbed::Part(HTMLPart::Text("Paragraph text.".into())),
                HTMLPartOrEmbed::Part(HTMLPart::Tag(HTMLTag {
                    name: "p",
                    kind: HTMLTagKind::Close,
                    attributes: Vec::new(),
                }))
            ]
        )
    }

    #[test]
    fn parse_embed_without_input() {
        assert_eq!(
            parse_embeds(vec![HTMLPart::Tag(HTMLTag {
                name: "run",
                kind: HTMLTagKind::Void,
                attributes: vec![("command", Some("date"))],
            })]),
            vec![HTMLPartOrEmbed::Embed(HTMLEmbed {
                command: "date",
                input: None,
            })]
        )
    }

    #[test]
    fn parse_embed_with_input() {
        assert_eq!(
            parse_embeds(vec![
                HTMLPart::Tag(HTMLTag {
                    name: "run",
                    kind: HTMLTagKind::Open,
                    attributes: vec![("command", Some("jq ."))],
                }),
                HTMLPart::Text("{\"number\": 42}".into()),
                HTMLPart::Tag(HTMLTag {
                    name: "run",
                    kind: HTMLTagKind::Close,
                    attributes: Vec::new(),
                }),
            ]),
            vec![HTMLPartOrEmbed::Embed(HTMLEmbed {
                command: "jq .",
                input: Some("{\"number\": 42}".into()),
            }),]
        )
    }

    #[test]
    fn parse_nested_embeds() {
        //
        // Nested embeds are currently not fully implemented.
        //
        // Only the top-level embed is parsed, treating the
        // inner embeds as an unexecuted string input.
        //
        assert_eq!(
            parse_embeds(vec![
                HTMLPart::Tag(HTMLTag {
                    name: "run",
                    kind: HTMLTagKind::Open,
                    attributes: vec![("command", Some("wc -l"))],
                }),
                HTMLPart::Tag(HTMLTag {
                    name: "run",
                    kind: HTMLTagKind::Open,
                    attributes: vec![("command", Some("jq ."))],
                }),
                HTMLPart::Text("{\"number\": 42}".into()),
                HTMLPart::Tag(HTMLTag {
                    name: "run",
                    kind: HTMLTagKind::Close,
                    attributes: Vec::new(),
                }),
                HTMLPart::Tag(HTMLTag {
                    name: "run",
                    kind: HTMLTagKind::Close,
                    attributes: Vec::new(),
                }),
            ]),
            vec![HTMLPartOrEmbed::Embed(HTMLEmbed {
                command: "wc -l",
                input: Some("<run command=\"jq .\">{\"number\": 42}</run>".into()),
            }),]
        )
    }
}
