use lazy_static::lazy_static;
use std::collections::HashSet;

lazy_static! {
    /// URI HTML attributes
    ///
    /// Used to find dependencies through relative URIs.
    ///
    pub static ref URI_HTML_ATTRIBUTES: HashSet<&'static str> = [
        "href",
        "src",
    ]
    .iter()
    .copied()
    .collect();
}
