use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{alpha1, char, multispace0, one_of, space1},
    combinator::opt,
    multi::many0,
    sequence::tuple,
    IResult,
};

use crate::types::{HTMLTag, HTMLTagKind};

impl<'a> HTMLTag<'a> {
    /// Parse HTML tag from string
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
        let (input, _) = multispace0(input)?;
        alt((attributes_tag, close_tag))(input)
    }
}

/// Tag name
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name
///
fn tag_name(input: &str) -> IResult<&str, &str> {
    let (input, name) = alpha1(input)?;
    Ok((input, name))
}

/// Attribute name
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attribute-name
///
fn attribute_name(input: &str) -> IResult<&str, &str> {
    alpha1(input)
}

/// Attribute value
///
/// Supports the four different types:
///
/// - Empty
/// - Unquoted
/// - Single quoted
/// - Double quoted
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attribute-value
///
fn attribute_value(input: &str) -> IResult<&str, Option<&str>> {
    alt((
        attribute_value_double_quoted,
        attribute_value_single_quoted,
        attribute_value_unquoted,
        attribute_value_empty,
    ))(input)
}

fn attribute_value_double_quoted(input: &str) -> IResult<&str, Option<&str>> {
    let (input, matches) = tuple((char('='), char('"'), is_not("\""), char('"')))(input)?;
    let (_, _, value, _) = matches;

    Ok((input, Some(value)))
}

fn attribute_value_single_quoted(input: &str) -> IResult<&str, Option<&str>> {
    let (input, matches) = tuple((char('='), char('\''), is_not("'"), char('\'')))(input)?;
    let (_, _, value, _) = matches;

    Ok((input, Some(value)))
}

fn attribute_value_unquoted(input: &str) -> IResult<&str, Option<&str>> {
    let (input, (_, value)) = tuple((char('='), is_not(" \u{0c}\t\r\n\"'=<>`")))(input)?;

    Ok((input, Some(value)))
}

fn attribute_value_empty(input: &str) -> IResult<&str, Option<&str>> {
    let _ = one_of(" \t\r\n>")(input)?;

    Ok((input, None))
}

/// Spaced attribute
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attributes
///
fn spaced_attribute(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    let (input, matches) = tuple((space1, attribute_name, attribute_value))(input)?;
    let (_, name, value) = matches;

    Ok((input, (name, value)))
}

fn void_delimiter(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((space1, char('/')))(input)?;
    Ok((input, ()))
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

fn attributes_tag(input: &str) -> IResult<&str, HTMLTag> {
    let (input, matches) = tuple((
        char('<'),
        tag_name,
        many0(spaced_attribute),
        opt(void_delimiter),
        char('>'),
    ))(input)?;
    let (_, name, attributes, void_delimiter, _) = matches;
    let kind = match void_delimiter {
        Some(_) => HTMLTagKind::Void,
        None => HTMLTagKind::Open,
    };

    Ok((
        input,
        HTMLTag {
            kind,
            name,
            attributes,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err};

    #[test]
    fn attribute_value_double_quoted() {
        assert_eq!(
            super::attribute_value("=\"some value\">"),
            Ok((">", Some("some value")))
        );
    }

    #[test]
    fn attribute_value_single_quoted() {
        assert_eq!(
            super::attribute_value("='some value'>"),
            Ok((">", Some("some value")))
        );
    }

    #[test]
    fn attribute_value_unquoted() {
        assert_eq!(
            super::attribute_value("=some-value>"),
            Ok((">", Some("some-value")))
        );
    }

    #[test]
    fn attribute_value_empty() {
        assert_eq!(super::attribute_value(" >"), Ok((" >", None)));
    }

    #[test]
    fn attribute_value_incomplete() {
        assert_eq!(
            super::attribute_value("= >"),
            Err(Err::Error(("= >", ErrorKind::OneOf)))
        );
    }

    #[test]
    fn open() {
        assert_eq!(
            HTMLTag::parse("<div>"),
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
    fn close() {
        assert_eq!(
            HTMLTag::parse("</div>"),
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
    fn void() {
        assert_eq!(
            HTMLTag::parse("<input />"),
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
            HTMLTag::parse("<div id=\"main\" class=\"layout\">"),
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
            HTMLTag::parse("<input type=\"radio\" class=\"custom-radio\" />"),
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
        assert_eq!(HTMLTag::parse(""), Err(Err::Error(("", ErrorKind::Char))));
    }

    #[test]
    fn close_with_attributes_error() {
        assert_eq!(
            HTMLTag::parse("</div id=\"main\" class=\"layout\">"),
            Err(Err::Error((
                " id=\"main\" class=\"layout\">",
                ErrorKind::Char
            )))
        );
    }
}
