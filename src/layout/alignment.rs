use std::collections::HashSet;

#[derive(Clone, Debug)]
pub enum HorizontalAlignment {
    Left,
    Center,
    Right
}

#[derive(Clone, Debug)]
pub enum VerticalAlignment {
    Top,
    Center,
    Bottom
}

#[derive(Clone, Debug)]
pub struct Alignment {
    vertical: VerticalAlignment,
    horizontal: HorizontalAlignment
}

#[derive(Debug, Clone, std::hash::Hash, PartialEq, Eq)]
pub enum Edge {
    Top,
    Right,
    Bottom,
    Left
}

impl Edge {
    pub fn all() -> HashSet<Edge> {
        dictionary!(Edge::Top, Edge::Right, Edge::Bottom, Edge::Left)
    }

    pub fn horizontal() -> HashSet<Edge> {
        dictionary!(Edge::Right, Edge::Left)
    }

    pub fn vertical() -> HashSet<Edge> {
        dictionary!(Edge::Top, Edge::Bottom)
    }
}

impl Alignment {
    pub fn center() -> Self {
        Self { vertical: VerticalAlignment::Center, horizontal: HorizontalAlignment::Center }
    }

    pub fn top_left() -> Self {
        Self {
            vertical: VerticalAlignment::Top,
            horizontal: HorizontalAlignment::Left
        }
    }
}