mod parse_attribute;
mod parse_comment;
mod parse_doctype;
mod parse_part;
mod parse_tag;

pub use parse_part::{parse_all_parts, parse_html};
pub use parse_tag::parse_tag;
