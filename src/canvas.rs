use crate::layout;

use layout::geometry::{Rect, Size};

#[derive(Default, Clone)]
pub enum TwoBitPixel {
    #[default]
    Zero,
    One
}

pub struct Canvas<Content: Default + Clone> {
    size: Size,
    contents: Vec<Content>
}

impl<Content: Default + Clone> Default for Canvas<Content> {
    fn default() -> Self {
        Self::new()
    }
}

impl<Content: Default + Clone> Canvas<Content> {
    pub fn new() -> Self {
        Canvas {
            size: Size::zero(),
            contents: Vec::new()
        }
    }

    pub fn create_in_bounds(size: &Size) -> Self {
        Canvas {
            size: size.clone(),
            contents: vec![
                Default::default();
                size.width * size.height
            ]
        }
    }

    pub fn create(width: usize, height: usize) -> Self {
        Canvas {
            size: Size::new(width, height),
            contents: vec![
                Default::default();
                width * height
            ]
        }
    }
}

impl<Content: Default + Clone> Canvas<Content> {
    pub fn width(&self) -> usize {
        self.size.width
    }

    pub fn height(&self) -> usize {
        self.size.height
    }
}

impl<Content: Default + Clone> Canvas<Content> {
    pub fn get_at(&self, x: usize, y: usize) -> Option<&Content> {
        if x >= self.size.width || y >= self.size.height {
            return None;
        }

        let index = y * self.size.width + x;

        Some(&self.contents[index])
    }

    pub fn write(&mut self, content: &Content, x: usize, y: usize) {
        if x >= self.size.width || y >= self.size.height { return; }

        let index = y * self.size.width + x;

        self.contents[index] = content.clone();
    }

    pub fn draw_rect(&mut self, bounds: &Rect, content: &Content) {
        for x in bounds.x..(bounds.x + bounds.width as i64) {
            for y in bounds.y..(bounds.y + bounds.height as i64) {
                if x < 0 || x >= self.size.width as i64 { continue; }
                if y < 0 || y >= self.size.height as i64 { continue; }

                self.write(content, x as usize, y as usize);
            }
        }
    }

    pub fn paste_canvas(&mut self, other: &Self, bounds: &Rect) {
        assert_eq!(other.size.width, bounds.width);
        assert_eq!(other.size.height, bounds.height);

        for x in 0..bounds.width {
            for y in 0..bounds.height {
                let content = match other.get_at(x, y) {
                    Some(content) => content,
                    None => continue
                };

                self.write(content, x + bounds.x as usize, y + bounds.y as usize);
            }
        }
    }

    pub fn clear_with(&mut self, content: &Content) {
        self.draw_rect(&Rect::from_size(&self.size), content);
    }
}