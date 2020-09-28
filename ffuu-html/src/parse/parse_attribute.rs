use nom::{
    branch::alt,
    bytes::complete::is_not,
    character::complete::{alpha1, char, one_of, space1},
    sequence::tuple,
    IResult,
};

/// Spaced attribute
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attributes
///
pub fn spaced_attribute(input: &str) -> IResult<&str, (&str, Option<&str>)> {
    let (input, matches) = tuple((space1, attribute_name, attribute_value))(input)?;
    let (_, name, value) = matches;

    Ok((input, (name, value)))
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
/// - Double quoted
/// - Single quoted
/// - Unquoted
/// - Empty
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

/// Attribute value -- double quoted
///
/// Example:
///
/// ```html
/// <div attribute="some value">
/// ```
///
fn attribute_value_double_quoted(input: &str) -> IResult<&str, Option<&str>> {
    let (input, matches) = tuple((char('='), char('"'), is_not("\""), char('"')))(input)?;
    let (_, _, value, _) = matches;

    Ok((input, Some(value)))
}

/// Attribute value -- single quoted
///
/// Example:
///
/// ```html
/// <div attribute='some value'>
/// ```
///
fn attribute_value_single_quoted(input: &str) -> IResult<&str, Option<&str>> {
    let (input, matches) = tuple((char('='), char('\''), is_not("'"), char('\'')))(input)?;
    let (_, _, value, _) = matches;

    Ok((input, Some(value)))
}

/// Attribute value -- unquoted
///
/// Example:
///
/// ```html
/// <div attribute=some-value>
/// ```
///
fn attribute_value_unquoted(input: &str) -> IResult<&str, Option<&str>> {
    let (input, (_, value)) = tuple((char('='), is_not(" \u{0c}\t\r\n\"'=<>`")))(input)?;

    Ok((input, Some(value)))
}

/// Attribute value -- empty
///
/// Example:
///
/// ```html
/// <div attribute>
/// ```
///
fn attribute_value_empty(input: &str) -> IResult<&str, Option<&str>> {
    let _ = one_of(" \t\r\n>")(input)?;

    Ok((input, None))
}

#[cfg(test)]
mod tests {
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
}
