use nom::{
    IResult,
    character::complete::{char, alphanumeric1},
};

/// Parse HTML tag from a string
pub fn parse_html_tag(input: &str) -> IResult<&str, ()> {
    let (input, _) = char('<')(input)?;
    let (input, _) = alphanumeric1(input)?;
    let (input, _) = char('>')(input)?;

    Ok((input, ()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use nom::{
      Err,
      error::ErrorKind,
    };

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
        assert_eq!(parse_html_tag("<div id=\"main\" class=\"layout\">"), Ok(("", ())));
    }

    #[test]
    fn void_with_attributes() {
        assert_eq!(parse_html_tag("<input type=\"radio\" class=\"custom-radio\" />"), Ok(("", ())));
    }

    #[test]
    fn empty_error() {
        assert_eq!(parse_html_tag(""), Err(Err::Error(("", ErrorKind::Char))));
    }

    #[test]
    fn close_with_attributes_error() {
        assert_eq!(parse_html_tag("</div id=\"main\" class=\"layout\">"), Err(Err::Error(("", ErrorKind::Char))));
    }
}