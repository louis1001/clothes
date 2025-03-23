use std::collections::HashSet;
use crate::layout::alignment::Edge;

use super::alignment;
use super::geometry;

#[derive(Clone, Debug)]
pub enum Node<Content: Clone + Default + std::fmt::Debug, Ctx: std::fmt::Debug> {
    Text(String, Content),
    Width(usize, Box<Node<Content, Ctx>>),
    Height(usize, Box<Node<Content, Ctx>>),
    TopPadding(usize, Box<Node<Content, Ctx>>),
    RightPadding(usize, Box<Node<Content, Ctx>>),
    BottomPadding(usize, Box<Node<Content, Ctx>>),
    LeftPadding(usize, Box<Node<Content, Ctx>>),
    VCenter(Box<Node<Content, Ctx>>),
    HCenter(Box<Node<Content, Ctx>>),
    VBottomAlign(Box<Node<Content, Ctx>>),
    HRightAlign(Box<Node<Content, Ctx>>),
    VTopAlign(Box<Node<Content, Ctx>>),
    HLeftAlign(Box<Node<Content, Ctx>>),
    Background(Content, Box<Node<Content, Ctx>>),
    TopBorder(usize, Content, Box<Node<Content, Ctx>>),
    BottomBorder(usize, Content, Box<Node<Content, Ctx>>),
    LeftBorder(usize, Content, Box<Node<Content, Ctx>>),
    RightBorder(usize, Content, Box<Node<Content, Ctx>>),

    VerticalStack(alignment::HorizontalAlignment, usize, Vec<Node<Content, Ctx>>),
    HorizontalStack(alignment::VerticalAlignment, usize, Vec<Node<Content, Ctx>>),

    // DrawCanvas(fn(&mut Ctx, &Rect)->crate::canvas::TextCanvas),
    WithContext(fn(&Ctx)->Node<Content, Ctx>)
}

impl<Content: Clone + Default + std::fmt::Debug, Ctx: Clone + std::fmt::Debug> Node<Content, Ctx> {
    pub fn text(text: &str) -> Node<Content, Ctx> {
        Node::Text(text.to_string(), Content::default())
    }

    pub fn text_with_content(text: &str, content: Content) -> Node<Content, Ctx> {
        Node::Text(text.to_string(), content)
    }

    pub fn center(self) -> Node<Content, Ctx> {
        Node::VCenter(Box::new(Node::HCenter(Box::new(self))))
    }

    pub fn center_vertically(self) -> Node<Content, Ctx> {
        Node::VCenter(Box::new(self))
    }

    pub fn center_horizontally(self) -> Node<Content, Ctx> {
        Node::HCenter(Box::new(self))
    }

    pub fn width(self, n: usize) -> Node<Content, Ctx> {
        Node::Width(n, Box::new(self))
    }
    
    pub fn height(self, n: usize) -> Node<Content, Ctx> {
        Node::Height(n, Box::new(self))
    }
    
    pub fn padding_top(self, n: usize) -> Node<Content, Ctx> {
        Node::TopPadding(n, Box::new(self))
    }
    
    pub fn padding_bottom(self, n: usize) -> Node<Content, Ctx> {
        Node::BottomPadding(n, Box::new(self))
    }

    pub fn padding_left(self, n: usize) -> Node<Content, Ctx> {
        Node::LeftPadding(n, Box::new(self))
    }
    
    pub fn padding_right(self, n: usize) -> Node<Content, Ctx> {
        Node::RightPadding(n, Box::new(self))
    }

    pub fn padding_horizontal(self, n: usize) -> Node<Content, Ctx> {
        self.padding_left(n).padding_right(n)
    }

    pub fn padding_vertical(self, n: usize) -> Node<Content, Ctx> {
        self.padding_top(n).padding_bottom(n)
    }

    pub fn padding(self, n: usize) -> Node<Content, Ctx> {
        self
            .padding_vertical(n)
            .padding_horizontal(n)
    }

    pub fn align_right(self) -> Node<Content, Ctx> {
        Node::HRightAlign(Box::new(self))
    }

    pub fn align_left(self) -> Node<Content, Ctx> {
        Node::HLeftAlign(Box::new(self))
    }

    pub fn align_top(self) -> Node<Content, Ctx> {
        Node::VTopAlign(Box::new(self))
    }

    pub fn align_bottom(self) -> Node<Content, Ctx> {
        Node::VBottomAlign(Box::new(self))
    }

    pub fn border<C: Into<Content> + Clone>(self, n: usize, c: C, edges: HashSet<alignment::Edge>) -> Node<Content, Ctx> {
        let mut resulting_node = self;

        for edge in edges {
            match edge {
                Edge::Top => resulting_node = Node::TopBorder(n, c.clone().into(), Box::new(resulting_node)),
                Edge::Bottom => resulting_node = Node::BottomBorder(n, c.clone().into(), Box::new(resulting_node)),
                Edge::Left => resulting_node = Node::LeftBorder(n, c.clone().into(), Box::new(resulting_node)),
                Edge::Right => resulting_node = Node::RightBorder(n, c.clone().into(), Box::new(resulting_node))
            }
        }
        
        resulting_node
    }

    pub fn background<C: Into<Content>>(self, c: C) -> Node<Content, Ctx> {
        Node::Background(c.into(), Box::new(self))
    }

    pub fn vertical_stack(nodes: Vec<Node<Content, Ctx>>) -> Node<Content, Ctx> {
        Node::VerticalStack(alignment::HorizontalAlignment::Center, 0, nodes)
    }
    
    pub fn horizontal_stack(nodes: Vec<Node<Content, Ctx>>) -> Node<Content, Ctx> {
        Node::HorizontalStack(alignment::VerticalAlignment::Center, 0, nodes)
    }

    pub fn grid<State, Item: Clone>(items: &geometry::Matrix<Item>, spacing: usize, view: fn(&Item)->Node<Content, Ctx>) -> Node<Content, Ctx> {
        let mut rows = vec![];

        let mut row = vec![];
        let mut col_counter = 0;
        for item in items.data() {
            col_counter += 1;

            let cell = view(item).center();
            row.push(cell);

            if col_counter == items.shape().0 {
                rows.push(Node::HorizontalStack(alignment::VerticalAlignment::Center, spacing, row));
                row = vec![];
                col_counter = 0;
            }
        }

        Node::VerticalStack(alignment::HorizontalAlignment::Center, spacing, rows)
    }
}