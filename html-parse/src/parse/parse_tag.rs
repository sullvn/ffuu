use nom::{
    branch::alt,
    character::complete::{alphanumeric1, char, multispace0, space1},
    combinator::opt,
    multi::many0,
    sequence::tuple,
    IResult,
};

use super::parse_attribute::spaced_attribute;
use crate::standard_elements::VOID_HTML_ELEMENTS;
use crate::types::{HTMLTag, HTMLTagKind};

pub fn parse_tag(input: &str) -> IResult<&str, HTMLTag> {
    let (input, _) = multispace0(input)?;
    alt((attributes_tag, close_tag))(input)
}

fn attributes_tag(input: &str) -> IResult<&str, HTMLTag> {
    let (input_rest, matches) = tuple((
        char('<'),
        tag_name,
        many0(spaced_attribute),
        opt(void_delimiter),
        char('>'),
    ))(input)?;
    let (_, name, attributes, void_delimiter, _) = matches;

    let has_void_name = VOID_HTML_ELEMENTS.contains(name);
    let has_void_delimiter = void_delimiter.is_some();
    let is_void = has_void_name || has_void_delimiter;
    let kind = match is_void {
        true => HTMLTagKind::Void,
        false => HTMLTagKind::Open,
    };

    Ok((
        input_rest,
        HTMLTag {
            kind,
            name,
            attributes,
        },
    ))
}

fn close_tag(input: &str) -> IResult<&str, HTMLTag> {
    let (input, matches) = tuple((char('<'), char('/'), tag_name, char('>')))(input)?;
    let (_, _, name, _) = matches;

    Ok((
        input,
        HTMLTag {
            kind: HTMLTagKind::Close,
            name,
            attributes: Vec::new(),
        },
    ))
}

/// Tag name
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name
///
fn tag_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = alphanumeric1(input)?;
    Ok((input, name))
}

fn void_delimiter(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((space1, char('/')))(input)?;
    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err};

    #[test]
    fn open() {
        assert_eq!(
            parse_tag("<div>"),
            Ok((
                "",
                HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "div",
                    attributes: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn open_alphanumeric() {
        assert_eq!(
            parse_tag("<h1>"),
            Ok((
                "",
                HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "h1",
                    attributes: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn close() {
        assert_eq!(
            parse_tag("</div>"),
            Ok((
                "",
                HTMLTag {
                    kind: HTMLTagKind::Close,
                    name: "div",
                    attributes: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn void_xhtml() {
        assert_eq!(
            parse_tag("<input />"),
            Ok((
                "",
                HTMLTag {
                    kind: HTMLTagKind::Void,
                    name: "input",
                    attributes: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn void_html() {
        assert_eq!(
            parse_tag("<input>"),
            Ok((
                "",
                HTMLTag {
                    kind: HTMLTagKind::Void,
                    name: "input",
                    attributes: Vec::new()
                }
            ))
        );
    }

    #[test]
    fn open_with_attributes() {
        assert_eq!(
            parse_tag("<div id=\"main\" class=\"layout\">"),
            Ok((
                "",
                HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "div",
                    attributes: vec![("id", Some("main")), ("class", Some("layout"))]
                }
            ))
        );
    }

    #[test]
    fn void_with_attributes() {
        assert_eq!(
            parse_tag("<input type=\"radio\" class=\"custom-radio\" />"),
            Ok((
                "",
                HTMLTag {
                    kind: HTMLTagKind::Void,
                    name: "input",
                    attributes: vec![("type", Some("radio")), ("class", Some("custom-radio"))]
                }
            ))
        );
    }

    #[test]
    fn empty_error() {
        assert_eq!(parse_tag(""), Err(Err::Error(("", ErrorKind::Char))));
    }

    #[test]
    fn close_with_attributes_error() {
        assert_eq!(
            parse_tag("</div id=\"main\" class=\"layout\">"),
            Err(Err::Error((
                " id=\"main\" class=\"layout\">",
                ErrorKind::Char
            )))
        );
    }
}
