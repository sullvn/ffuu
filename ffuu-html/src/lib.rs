mod parse;
mod standard_elements;
mod types;

pub use parse::{parse_all_parts, parse_tag};
pub use standard_elements::STANDARD_HTML_ELEMENTS;
pub use types::{HTMLTag, HTMLTagKind};
