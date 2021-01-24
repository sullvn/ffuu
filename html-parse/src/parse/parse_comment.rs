use nom::{
    bytes::complete::{tag, take_until},
    sequence::tuple,
    IResult,
};

use crate::types::HTMLPart;

/// Parse HTML comment
///
/// Is forgiving of comments with bad contents which don't fit the spec.
///
/// Spec: https://html.spec.whatwg.org/multipage/syntax.html#comments
///
pub fn parse_comment_part(input: &str) -> IResult<&str, HTMLPart> {
    let comment_open = "<!--";
    let comment_close = "-->";

    let (input, (_, text, _)) = tuple((
        tag(comment_open),
        take_until(comment_close),
        tag(comment_close),
    ))(input)?;

    Ok((input, HTMLPart::Comment(text)))
}

#[cfg(test)]
mod tests {
    use super::parse_comment_part;
    use crate::HTMLPart;

    #[test]
    fn comment_empty() {
        assert_eq!(
            parse_comment_part("<!---->"),
            Ok(("", HTMLPart::Comment("")))
        );
    }

    #[test]
    fn comment_with_text() {
        assert_eq!(
            parse_comment_part("<!-- Here's a comment\n with a newline -->"),
            Ok(("", HTMLPart::Comment(" Here's a comment\n with a newline ")))
        );
    }

    #[test]
    fn comment_with_bad_contents() {
        assert_eq!(
            parse_comment_part("<!--><!--->"),
            Ok(("", HTMLPart::Comment("><!-")))
        );
    }
}
