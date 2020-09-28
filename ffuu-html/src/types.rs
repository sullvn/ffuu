#[derive(Debug, Eq, PartialEq)]
pub enum HTMLTagKind {
    Open,
    Close,
    Void,
}

impl HTMLTagKind {
    pub fn depth_change(&self) -> isize {
        match self {
            HTMLTagKind::Open => 1,
            HTMLTagKind::Void => 0,
            HTMLTagKind::Close => -1,
        }
    }
}

#[derive(Debug, Eq, PartialEq)]
pub struct HTMLTag<'a> {
    pub kind: HTMLTagKind,
    pub name: &'a str,
    pub attributes: Vec<(&'a str, Option<&'a str>)>,
}

#[derive(Debug, Eq, PartialEq)]
pub enum HTMLPart<'a> {
    Tag(HTMLTag<'a>),
    Other(&'a str),
}
