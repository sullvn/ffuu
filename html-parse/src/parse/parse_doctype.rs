use nom::{
    bytes::complete::{is_not, tag, tag_no_case},
    character::complete::{char, multispace1},
    sequence::tuple,
    IResult,
};

use crate::types::HTMLPart;

/// Parse HTML DOCTYPE
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#the-doctype
///
pub fn parse_doctype_part(input: &str) -> IResult<&str, HTMLPart> {
    let (input, _) = tuple((
        tag("<!"),
        tag_no_case("doctype"),
        multispace1,
        is_not(">"),
        char('>'),
    ))(input)?;
    Ok((input, HTMLPart::DocType))
}

#[cfg(test)]
mod tests {
    use super::parse_doctype_part;
    use crate::HTMLPart;

    #[test]
    fn doctype_upper() {
        assert_eq!(
            parse_doctype_part("<!DOCTYPE html>"),
            Ok(("", HTMLPart::DocType))
        );
    }

    #[test]
    fn doctype_lower() {
        assert_eq!(
            parse_doctype_part("<!doctype html>"),
            Ok(("", HTMLPart::DocType))
        );
    }

    #[test]
    fn doctype_with_legacy() {
        assert_eq!(
            parse_doctype_part("<!DOCTYPE html SYSTEM \"about:legacy-compat\">"),
            Ok(("", HTMLPart::DocType))
        );
    }
}
