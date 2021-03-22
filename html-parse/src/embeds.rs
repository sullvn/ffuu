use crate::{HTMLPart, HTMLTag, HTMLTagKind};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct HTMLEmbed<'a> {
    command: &'a str,
    input: Option<&'a str>,
}

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
pub fn parse_embeds<'a>(html_parts: Vec<HTMLPart<'a>>) -> Vec<HTMLPartOrEmbed<'a>> {
    let mut html_parts_or_embeds = Vec::new();
    let mut maybe_pending_embed: Option<HTMLEmbed<'a>> = None;

    for hp in html_parts.into_iter() {
        let maybe_new_embed: Option<HTMLEmbed<'a>> = (&hp).into();
        let is_new_embed_end: bool = match &hp {
            HTMLPart::Tag(HTMLTag {
                name: "run",
                kind: HTMLTagKind::Close,
                ..
            }) => true,
            _ => false,
        };

        match (&maybe_pending_embed, maybe_new_embed, is_new_embed_end) {
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
