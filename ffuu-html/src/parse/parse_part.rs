use nom::{branch::alt, bytes::complete::is_not, combinator::all_consuming, multi::many0, IResult};

use super::parse_comment::parse_comment_part;
use super::parse_tag::parse_tag;
use crate::types::HTMLPart;

pub fn parse_all_parts(input: &str) -> IResult<&str, Vec<HTMLPart>> {
    all_consuming(many0(parse_part))(input)
}

fn parse_part(input: &str) -> IResult<&str, HTMLPart> {
    alt((parse_tag_part, parse_text_part, parse_comment_part))(input)
}

fn parse_tag_part(input: &str) -> IResult<&str, HTMLPart> {
    let (input, tag) = parse_tag(input)?;
    Ok((input, HTMLPart::Tag(tag)))
}

fn parse_text_part(input: &str) -> IResult<&str, HTMLPart> {
    let (input, text) = is_not("<")(input)?;

    Ok((input, HTMLPart::Text(text)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{HTMLTag, HTMLTagKind};

    #[test]
    fn only_text() {
        assert_eq!(
            parse_all_parts("No tags, just text."),
            Ok(("", vec![HTMLPart::Text("No tags, just text.")]))
        );
    }

    #[test]
    fn tag_with_text() {
        assert_eq!(
            parse_all_parts("Outside <p class=\"test\" toggle>Some content.</p> text"),
            Ok((
                "",
                vec![
                    HTMLPart::Text("Outside "),
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Open,
                        name: "p",
                        attributes: vec![("class", Some("test")), ("toggle", None)],
                    }),
                    HTMLPart::Text("Some content."),
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Close,
                        name: "p",
                        attributes: vec![],
                    }),
                    HTMLPart::Text(" text")
                ]
            ))
        );
    }

    #[test]
    fn nested_tags() {
        assert_eq!(
            parse_all_parts("<form><label>Radio</label><input type=\"radio\"></form>"),
            Ok((
                "",
                vec![
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Open,
                        name: "form",
                        attributes: vec![],
                    }),
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Open,
                        name: "label",
                        attributes: vec![],
                    }),
                    HTMLPart::Text("Radio"),
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Close,
                        name: "label",
                        attributes: vec![],
                    }),
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Void,
                        name: "input",
                        attributes: vec![("type", Some("radio"))],
                    }),
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Close,
                        name: "form",
                        attributes: vec![],
                    }),
                ]
            ))
        );
    }
}
