use crate::{HTMLPart, HTMLTag, HTMLTagKind};

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct HTMLEmbed<'a> {
    command: &'a str,
    input: Option<&'a str>,
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
                kind: HTMLTagKind::Open,
                attributes,
            }) => attributes
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

#[allow(dead_code)]
struct PendingHTMLEmbed<'a> {
    command: &'a str,
    depth: usize,
    input_parts: Vec<HTMLPart<'a>>,
}

#[allow(dead_code)]
pub fn parse_embeds<'a>(html_parts: Vec<HTMLPart<'a>>) -> Vec<HTMLPartOrEmbed<'a>> {
    let mut html_parts_or_embeds = Vec::new();
    // TODO, Change pending embed to:
    // - Command str
    // - Depth of embed
    // - Inner HTML parts (to be formatted as stdin)
    let mut maybe_pending_embed: Option<HTMLEmbed<'a>> = None;

    // TODO: Create iterator which attaches HTML depth
    for hp in html_parts.into_iter() {
        let maybe_new_embed: Option<HTMLEmbed<'a>> = (&hp).into();
        let is_embed_end: bool = match &hp {
            HTMLPart::Tag(HTMLTag {
                name: "run",
                kind: HTMLTagKind::Close,
                ..
            }) => true,
            _ => false,
        };

        match (&maybe_pending_embed, maybe_new_embed, is_embed_end) {
            (None, Some(new_embed), false) => maybe_pending_embed = Some(new_embed),
            (Some(_pending_embed), _, false) => todo!("Merge part into pending embed input"),
            (Some(finished_embed), None, true) => {
                html_parts_or_embeds.push(HTMLPartOrEmbed::Embed(finished_embed.clone()));
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
                HTMLPart::Text("Paragraph text."),
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
                HTMLPartOrEmbed::Part(HTMLPart::Text("Paragraph text.")),
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
                HTMLPart::Text("{\"number\": 42}"),
                HTMLPart::Tag(HTMLTag {
                    name: "run",
                    kind: HTMLTagKind::Close,
                    attributes: Vec::new(),
                }),
            ]),
            vec![HTMLPartOrEmbed::Embed(HTMLEmbed {
                command: "jq .",
                input: Some("{\"number\": 42}")
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
                HTMLPart::Text("{\"number\": 42}"),
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
                input: Some(
                    "\
                    <run command=\"jq .\">
                      {\"number\": 42}
                    </run>
                    "
                ),
            }),]
        )
    }
}
