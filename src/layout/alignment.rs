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

impl Alignment {
    pub fn vertical(&self) -> &VerticalAlignment {
        &self.vertical
    }

    pub fn horizontal(&self) -> &HorizontalAlignment {
        &self.horizontal
    }
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

    pub fn top_right() -> Self {
        Self {
            vertical: VerticalAlignment::Top,
            horizontal: HorizontalAlignment::Right
        }
    }

    pub fn bottom_left() -> Self {
        Self {
            vertical: VerticalAlignment::Bottom,
            horizontal: HorizontalAlignment::Left
        }
    }

    pub fn bottom_right() -> Self {
        Self {
            vertical: VerticalAlignment::Bottom,
            horizontal: HorizontalAlignment::Right
        }
    }

    pub fn top() -> Self {
        Self {
            vertical: VerticalAlignment::Top,
            horizontal: HorizontalAlignment::Center
        }
    }
    
    pub fn left() -> Self {
        Self {
            vertical: VerticalAlignment::Center,
            horizontal: HorizontalAlignment::Left
        }
    }

    pub fn right() -> Self {
        Self {
            vertical: VerticalAlignment::Center,
            horizontal: HorizontalAlignment::Right
        }
    }

    pub fn bottom() -> Self {
        Self {
            vertical: VerticalAlignment::Bottom,
            horizontal: HorizontalAlignment::Center
        }
    }
}