use std::{collections::HashSet, fmt::Debug};
use crate::fonts::Font;

use super::{alignment, geometry::Shape, node::{DetachedBehavior, ShapeBehavior}, sizing};

#[derive(Clone, Debug)]
pub enum SizedItem<Content: Clone + Default + Debug> {
    Text(String, Content, &'static Font),
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
    Detached(SizedNode<Content>, alignment::Alignment, DetachedBehavior, SizedNode<Content>),
    Border(usize, Content, HashSet<alignment::Edge>, SizedNode<Content>),

    VerticalStack(alignment::HorizontalAlignment, usize, Vec<SizedNode<Content>>),
    HorizontalStack(alignment::VerticalAlignment, usize, Vec<SizedNode<Content>>),

    Shape(Shape, ShapeBehavior, Content)
}

#[derive(Clone, Debug)]
pub struct SizedNode<Content: Clone + Default + Debug> {
    pub node: Box<SizedItem<Content>>,
    pub sizing: sizing::ItemSizing
}

impl<Content: Clone + Default + Debug> SizedNode<Content> {
    pub fn new(node: SizedItem<Content>, sizing: sizing::ItemSizing) -> Self {
        SizedNode { node: Box::new(node), sizing }
    }
}