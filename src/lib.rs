use nom::{
    branch::alt,
    character::complete::{alpha1, char, none_of, space1},
    combinator::opt,
    multi::many0,
    sequence::tuple,
    IResult,
};

/// Tag name
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-tag-name
///
fn tag_name(input: &str) -> IResult<&str, ()> {
    let (input, _) = alpha1(input)?;
    Ok((input, ()))
}

/// Attribute name
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attribute-name
///
fn attribute_name(input: &str) -> IResult<&str, ()> {
    let (input, _) = alpha1(input)?;
    Ok((input, ()))
}

/// Attribute value
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attribute-value
///
fn attribute_value(input: &str) -> IResult<&str, ()> {
    let (input, _) = many0(none_of("\""))(input)?;
    Ok((input, ()))
}

/// Spaced attribute
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#syntax-attributes
///
fn spaced_attribute(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((
        space1,
        attribute_name,
        char('='),
        char('"'),
        attribute_value,
        char('"'),
    ))(input)?;

    Ok((input, ()))
}

fn void_delimiter(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((space1, char('/')))(input)?;
    Ok((input, ()))
}

fn close_tag(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((char('<'), char('/'), tag_name, char('>')))(input)?;

    Ok((input, ()))
}

fn attributes_tag(input: &str) -> IResult<&str, ()> {
    let (input, _) = tuple((
        char('<'),
        tag_name,
        many0(spaced_attribute),
        opt(void_delimiter),
        char('>'),
    ))(input)?;

    Ok((input, ()))
}

/// Parse HTML tag from a string
pub fn parse_html_tag(input: &str) -> IResult<&str, ()> {
    alt((attributes_tag, close_tag))(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{error::ErrorKind, Err};

    #[test]
    fn attribute_value() {
        assert_eq!(super::attribute_value("a-class-name\""), Ok(("\"", ())));
    }

    #[test]
    fn open() {
        assert_eq!(parse_html_tag("<div>"), Ok(("", ())));
    }

    #[test]
    fn close() {
        assert_eq!(parse_html_tag("</div>"), Ok(("", ())));
    }

    #[test]
    fn void() {
        assert_eq!(parse_html_tag("<input />"), Ok(("", ())));
    }

    #[test]
    fn open_with_attributes() {
        assert_eq!(
            parse_html_tag("<div id=\"main\" class=\"layout\">"),
            Ok(("", ()))
        );
    }

    #[test]
    fn void_with_attributes() {
        assert_eq!(
            parse_html_tag("<input type=\"radio\" class=\"custom-radio\" />"),
            Ok(("", ()))
        );
    }

    #[test]
    fn empty_error() {
        assert_eq!(parse_html_tag(""), Err(Err::Error(("", ErrorKind::Char))));
    }

    #[test]
    fn close_with_attributes_error() {
        assert_eq!(
            parse_html_tag("</div id=\"main\" class=\"layout\">"),
            Err(Err::Error((
                " id=\"main\" class=\"layout\">",
                ErrorKind::Char
            )))
        );
    }
}
