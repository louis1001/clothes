use std::collections::HashSet;
use super::{alignment, sizing};

#[derive(Clone)]
pub enum SizedItem<Content: Clone + Default> {
    Text(String),
    Width(usize, SizedNode<Content>),
    Height(usize, SizedNode<Content>),
    TopPadding(usize, SizedNode<Content>),
    RightPadding(usize, SizedNode<Content>),
    BottomPadding(usize, SizedNode<Content>),
    LeftPadding(usize, SizedNode<Content>),
    VCenter(SizedNode<Content>),
    HCenter(SizedNode<Content>),
    VBottomAlign(SizedNode<Content>),
    HRightAlign(SizedNode<Content>),
    VTopAlign(SizedNode<Content>),
    HLeftAlign(SizedNode<Content>),
    Background(Content, SizedNode<Content>),
    Border(usize, Content, HashSet<alignment::Edge>, SizedNode<Content>),

    VerticalStack(alignment::HorizontalAlignment, usize, Vec<SizedNode<Content>>),
    HorizontalStack(alignment::VerticalAlignment, usize, Vec<SizedNode<Content>>)
}

#[derive(Clone)]
pub struct SizedNode<Content: Clone + Default> {
    pub node: Box<SizedItem<Content>>,
    pub sizing: sizing::ItemSizing
}

impl<Content: Clone + Default> SizedNode<Content> {
    pub fn new(node: SizedItem<Content>, sizing: sizing::ItemSizing) -> Self {
        SizedNode { node: Box::new(node), sizing }
    }
}