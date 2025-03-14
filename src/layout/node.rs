use std::collections::HashSet;
use super::alignment;
use super::geometry;

#[derive(Clone)]
pub enum Layout<Ctx> {
    Text(String),
    Width(usize, Box<Layout<Ctx>>),
    Height(usize, Box<Layout<Ctx>>),
    TopPadding(usize, Box<Layout<Ctx>>),
    RightPadding(usize, Box<Layout<Ctx>>),
    BottomPadding(usize, Box<Layout<Ctx>>),
    LeftPadding(usize, Box<Layout<Ctx>>),
    VCenter(Box<Layout<Ctx>>),
    HCenter(Box<Layout<Ctx>>),
    VBottomAlign(Box<Layout<Ctx>>),
    HRightAlign(Box<Layout<Ctx>>),
    VTopAlign(Box<Layout<Ctx>>),
    HLeftAlign(Box<Layout<Ctx>>),
    Background(char, Box<Layout<Ctx>>),
    Border(usize, char, HashSet<alignment::Edge>, Box<Layout<Ctx>>),

    VerticalStack(alignment::HorizontalAlignment, usize, Vec<Layout<Ctx>>),
    HorizontalStack(alignment::VerticalAlignment, usize, Vec<Layout<Ctx>>),

    // DrawCanvas(fn(&mut Ctx, &Rect)->crate::canvas::TextCanvas),
    WithContext(fn(&Ctx)->Layout<Ctx>)
}

impl<Ctx: Clone> Layout<Ctx> {
    pub fn text(content: &str) -> Layout<Ctx> {
        Layout::Text(content.to_string())
    }

    pub fn center(self) -> Layout<Ctx> {
        Layout::VCenter(Box::new(Layout::HCenter(Box::new(self))))
    }

    pub fn center_vertically(self) -> Layout<Ctx> {
        Layout::VCenter(Box::new(self))
    }

    pub fn center_horizontally(self) -> Layout<Ctx> {
        Layout::HCenter(Box::new(self))
    }

    pub fn width(self, n: usize) -> Layout<Ctx> {
        Layout::Width(n, Box::new(self))
    }
    
    pub fn height(self, n: usize) -> Layout<Ctx> {
        Layout::Height(n, Box::new(self))
    }
    
    pub fn padding_top(self, n: usize) -> Layout<Ctx> {
        Layout::TopPadding(n, Box::new(self))
    }
    
    pub fn padding_bottom(self, n: usize) -> Layout<Ctx> {
        Layout::BottomPadding(n, Box::new(self))
    }

    pub fn padding_left(self, n: usize) -> Layout<Ctx> {
        Layout::LeftPadding(n, Box::new(self))
    }
    
    pub fn padding_right(self, n: usize) -> Layout<Ctx> {
        Layout::RightPadding(n, Box::new(self))
    }

    pub fn padding_horizontal(self, n: usize) -> Layout<Ctx> {
        self.padding_left(n).padding_right(n)
    }

    pub fn padding_vertical(self, n: usize) -> Layout<Ctx> {
        self.padding_top(n).padding_bottom(n)
    }

    pub fn padding(self, n: usize) -> Layout<Ctx> {
        self
            .padding_vertical(n)
            .padding_horizontal(n)
    }

    pub fn align_right(self) -> Layout<Ctx> {
        Layout::HRightAlign(Box::new(self))
    }

    pub fn align_left(self) -> Layout<Ctx> {
        Layout::HLeftAlign(Box::new(self))
    }

    pub fn align_top(self) -> Layout<Ctx> {
        Layout::VTopAlign(Box::new(self))
    }

    pub fn align_bottom(self) -> Layout<Ctx> {
        Layout::VBottomAlign(Box::new(self))
    }

    pub fn border(self, n: usize, c: char, edges: HashSet<alignment::Edge>) -> Layout<Ctx> {
        Layout::Border(n, c, edges, Box::new(self))
    }

    pub fn background(self, c: char) -> Layout<Ctx> {
        Layout::Background(c, Box::new(self))
    }

    pub fn vertical_stack(nodes: Vec<Layout<Ctx>>) -> Layout<Ctx> {
        Layout::VerticalStack(alignment::HorizontalAlignment::Center, 0, nodes)
    }
    
    pub fn horizontal_stack(nodes: Vec<Layout<Ctx>>) -> Layout<Ctx> {
        Layout::HorizontalStack(alignment::VerticalAlignment::Center, 0, nodes)
    }

    pub fn grid<State, Item: Clone>(items: &geometry::Matrix<Item>, spacing: usize, view: fn(&Item)->Layout<Ctx>) -> Layout<Ctx> {
        let mut rows = vec![];

        let mut row = vec![];
        let mut col_counter = 0;
        for item in items.data() {
            col_counter += 1;

            let cell = view(item).center();
            row.push(cell);

            if col_counter == items.shape().0 {
                rows.push(Layout::HorizontalStack(alignment::VerticalAlignment::Center, spacing, row));
                row = vec![];
                col_counter = 0;
            }
        }

        Layout::VerticalStack(alignment::HorizontalAlignment::Center, spacing, rows)
    }
}