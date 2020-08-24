use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{alpha1, char, space1},
    combinator::opt,
    multi::many0,
    sequence::tuple,
    IResult,
};

#[derive(Debug, Eq, PartialEq)]
pub enum HTMLTagKind {
    Open,
    Close,
    Void,
}

impl HTMLTagKind {
    pub fn depth_change(&self) -> isize {
        match self {
            HTMLTagKind::Open => 1,
            HTMLTagKind::Void => 0,
            HTMLTagKind::Close => -1,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct HTMLTag<'a> {
    pub kind: HTMLTagKind,
    pub name: &'a str,
    pub attributes: Vec<(&'a str, &'a str)>,
}

impl<'a> HTMLTag<'a> {
    /// Parse HTML tag from string
    pub fn parse(input: &'a str) -> IResult<&'a str, Self> {
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
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attribute-value
///
/// TODO: Return string slice
///
fn attribute_value(input: &str) -> IResult<&str, &str> {
    is_not("\"")(input)
}

/// Spaced attribute
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attributes
///
fn spaced_attribute(input: &str) -> IResult<&str, (&str, &str)> {
    let (input, matches) = tuple((
        space1,
        attribute_name,
        char('='),
        char('"'),
        attribute_value,
        char('"'),
    ))(input)?;
    let (_, name, _, _, value, _) = matches;

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
    fn attribute_value() {
        assert_eq!(
            super::attribute_value("a-class-name\""),
            Ok(("\"", "a-class-name"))
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
                    attributes: vec![("id", "main"), ("class", "layout")]
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
                    attributes: vec![("type", "radio"), ("class", "custom-radio")]
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
