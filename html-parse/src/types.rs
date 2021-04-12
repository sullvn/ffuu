use crate::depth::DepthChange;
use std::borrow::Cow;

#[derive(Debug, Eq, PartialEq)]
pub enum HTMLTagKind {
    Open,
    Close,
    Void,
}

impl DepthChange for HTMLTagKind {
    fn depth_change(&self) -> isize {
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

impl<'a> DepthChange for HTMLTag<'a> {
    fn depth_change(&self) -> isize {
        self.kind.depth_change()
    }
}

#[derive(Debug, Eq, PartialEq)]
pub enum HTMLPart<'a> {
    Comment(&'a str),
    DocType,
    Tag(HTMLTag<'a>),
    Text(Cow<'a, str>),
}

impl<'a> DepthChange for HTMLPart<'a> {
    fn depth_change(&self) -> isize {
        match self {
            HTMLPart::Tag(tag) => tag.depth_change(),
            _ => 0,
        }
    }
}
