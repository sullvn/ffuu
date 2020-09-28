use nom::{branch::alt, bytes::complete::is_not, combinator::all_consuming, multi::many0, IResult};

use super::parse_tag::parse_tag;
use crate::types::HTMLPart;

pub fn parse_all_parts(input: &str) -> IResult<&str, Vec<HTMLPart>> {
    all_consuming(many0(parse_part))(input)
}

fn parse_part(input: &str) -> IResult<&str, HTMLPart> {
    alt((parse_other, parse_tag_wrapped))(input)
}

fn parse_tag_wrapped(input: &str) -> IResult<&str, HTMLPart> {
    let (input, tag) = parse_tag(input)?;
    Ok((input, HTMLPart::Tag(tag)))
}

fn parse_other(input: &str) -> IResult<&str, HTMLPart> {
    let (input, text) = is_not("<")(input)?;

    Ok((input, HTMLPart::Other(text)))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{HTMLTag, HTMLTagKind};

    #[test]
    fn tag_with_text() {
        assert_eq!(
            parse_all_parts("Outside <p class=\"test\" toggle>Some content.</p> text"),
            Ok((
                "",
                vec![
                    HTMLPart::Other("Outside "),
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Open,
                        name: "p",
                        attributes: vec![("class", Some("test")), ("toggle", None)],
                    }),
                    HTMLPart::Other("Some content."),
                    HTMLPart::Tag(HTMLTag {
                        kind: HTMLTagKind::Close,
                        name: "p",
                        attributes: vec![],
                    }),
                    HTMLPart::Other(" text")
                ]
            ))
        );
    }
}