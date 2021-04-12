use crate::depth::DepthChange;
use crate::{HTMLPart, HTMLTag, HTMLTagKind};

const INDENT: &str = "  ";

pub fn format_html<'a, T>(html_parts: T) -> String
where
    T: IntoIterator<Item = &'a HTMLPart<'a>>,
{
    let mut output = String::new();
    let mut depth: isize = 0;
    let mut inside_text: Option<isize> = None;

    for (i, hp) in html_parts.into_iter().enumerate() {
        let new_depth = if let HTMLPart::Tag(tag) = &hp {
            (depth + tag.kind.depth_change()).max(0)
        } else {
            depth
        };

        match (inside_text, &hp) {
            (
                Some(text_depth),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    ..
                }),
            ) if text_depth == depth => {
                inside_text = None;
            }
            (None, HTMLPart::Text(..)) => {
                inside_text = Some(depth);
            }
            (
                None,
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    ..
                }),
            ) => {
                if 0 < i {
                    output.push('\n');
                }
                output.push_str(&INDENT.repeat(new_depth as usize));
            }
            (None, _) => {
                if 0 < i {
                    output.push('\n');
                }
                output.push_str(&INDENT.repeat(depth as usize));
            }
            _ => {}
        };

        depth = new_depth;
        output.push_str(&format_html_part(&hp));
    }

    output
}

fn format_html_part(part: &HTMLPart) -> String {
    match part {
        HTMLPart::Comment(comment) => format!("<!--{}-->", comment),
        HTMLPart::DocType => "<!DOCTYPE html>".into(),
        HTMLPart::Tag(tag) => format_html_tag(tag),
        HTMLPart::Text(text) => (*text).into(),
    }
}

fn format_html_tag(tag: &HTMLTag) -> String {
    if tag.kind == HTMLTagKind::Close {
        return format!("</{}>", tag.name);
    }

    let attributes = tag
        .attributes
        .iter()
        .map(format_html_attribute)
        .collect::<Vec<String>>()
        .join(" ");

    let name_attr_sep = if tag.attributes.is_empty() { "" } else { " " };
    let void_slash = if tag.kind == HTMLTagKind::Void {
        " /"
    } else {
        ""
    };

    format!(
        "<{}{}{}{}>",
        tag.name, name_attr_sep, attributes, void_slash
    )
}

fn format_html_attribute(attribute: &(&str, Option<&str>)) -> String {
    let (name, maybe_value) = attribute;

    match maybe_value {
        Some(value) => format!("{}=\"{}\"", name, value),
        None => (*name).into(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn format_doctype() {
        assert_eq!(format_html(&vec![HTMLPart::DocType]), "<!DOCTYPE html>");
    }

    #[test]
    fn format_comment() {
        assert_eq!(
            format_html(&vec![HTMLPart::Comment("BORKEN")]),
            "<!--BORKEN-->"
        );
    }

    #[test]
    fn format_text() {
        assert_eq!(
            format_html(&vec![HTMLPart::Text("This is a paragraph, \nwhat of it.")]),
            "This is a paragraph, \nwhat of it."
        );
    }

    #[test]
    fn format_open_tag() {
        assert_eq!(
            format_html(&vec![HTMLPart::Tag(HTMLTag {
                kind: HTMLTagKind::Open,
                name: "span",
                attributes: vec![],
            })]),
            "<span>"
        );
    }

    #[test]
    fn format_open_attributes_tag() {
        assert_eq!(
            format_html(&vec![HTMLPart::Tag(HTMLTag {
                kind: HTMLTagKind::Open,
                name: "span",
                attributes: vec![("class", Some("alert"))],
            })]),
            "<span class=\"alert\">"
        );
    }

    #[test]
    fn format_open_boolean_attributes_tag() {
        assert_eq!(
            format_html(&vec![HTMLPart::Tag(HTMLTag {
                kind: HTMLTagKind::Open,
                name: "button",
                attributes: vec![("disabled", None)],
            })]),
            "<button disabled>"
        );
    }

    #[test]
    fn format_void_tag() {
        assert_eq!(
            format_html(&vec![HTMLPart::Tag(HTMLTag {
                kind: HTMLTagKind::Void,
                name: "meta",
                attributes: vec![],
            })]),
            "<meta />"
        );
    }

    #[test]
    fn format_void_attributes_tag() {
        assert_eq!(
            format_html(&vec![HTMLPart::Tag(HTMLTag {
                kind: HTMLTagKind::Void,
                name: "meta",
                attributes: vec![("charset", Some("utf-8"))],
            })]),
            "<meta charset=\"utf-8\" />"
        );
    }

    #[test]
    fn format_void_boolean_attributes_tag() {
        assert_eq!(
            format_html(&vec![HTMLPart::Tag(HTMLTag {
                kind: HTMLTagKind::Void,
                name: "button",
                attributes: vec![("disabled", None)],
            })]),
            "<button disabled />"
        );
    }

    #[test]
    fn format_close_tag() {
        assert_eq!(
            format_html(&vec![HTMLPart::Tag(HTMLTag {
                kind: HTMLTagKind::Close,
                name: "div",
                attributes: vec![],
            })]),
            "</div>"
        );
    }

    #[test]
    fn format_html_doc() {
        assert_eq!(
            format_html(&vec![
                HTMLPart::DocType,
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "html",
                    attributes: vec![],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "header",
                    attributes: vec![],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "title",
                    attributes: vec![],
                }),
                HTMLPart::Text("Title"),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    name: "title",
                    attributes: vec![],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Void,
                    name: "link",
                    attributes: vec![("rel", Some("stylesheet")), ("href", Some("./styles.css"))],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    name: "header",
                    attributes: vec![],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "body",
                    attributes: vec![],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "h1",
                    attributes: vec![],
                }),
                HTMLPart::Text("Header"),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    name: "h1",
                    attributes: vec![],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "p",
                    attributes: vec![],
                }),
                HTMLPart::Text("Two lines"),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Void,
                    name: "br",
                    attributes: vec![],
                }),
                HTMLPart::Text("of "),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Open,
                    name: "em",
                    attributes: vec![],
                }),
                HTMLPart::Text("text"),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    name: "em",
                    attributes: vec![],
                }),
                HTMLPart::Text("."),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    name: "p",
                    attributes: vec![],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    name: "body",
                    attributes: vec![],
                }),
                HTMLPart::Tag(HTMLTag {
                    kind: HTMLTagKind::Close,
                    name: "html",
                    attributes: vec![],
                }),
            ]),
            "\
<!DOCTYPE html>
<html>
  <header>
    <title>Title</title>
    <link rel=\"stylesheet\" href=\"./styles.css\" />
  </header>
  <body>
    <h1>Header</h1>
    <p>Two lines<br />of <em>text</em>.</p>
  </body>
</html>"
        );
    }
}
