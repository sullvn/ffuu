mod parse;
mod standard_attributes;
mod standard_elements;
mod types;

pub use parse::{parse_all_parts, parse_tag};
pub use standard_attributes::URI_HTML_ATTRIBUTES;
pub use standard_elements::STANDARD_HTML_ELEMENTS;
pub use types::{HTMLPart, HTMLTag, HTMLTagKind};
