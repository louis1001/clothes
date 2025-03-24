pub mod conversions;

use crate::layout::geometry::{Rect, Size};

pub struct Canvas<Content: Default + Clone> {
    pub(crate) size: Size,
    pub(crate) contents: Vec<Content>
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

    pub fn create_with_content(contents: Vec<Content>, size: &Size) -> Self {
        Canvas { size: size.clone(), contents }
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

    pub fn bounds(&self) -> Rect {
        Rect::from_size(&self.size)
    }
}

impl<Content: Default + Clone> Canvas<Content> {
    pub(crate) fn get_at(&self, x: usize, y: usize) -> Option<&Content> {
        if x >= self.size.width || y >= self.size.height {
            return None;
        }

        let index = y * self.size.width + x;

        Some(&self.contents[index])
    }

    pub fn write<C: Clone>(&mut self, content: &C, x: usize, y: usize) where Content: From<C> {
        if x >= self.size.width || y >= self.size.height { return; }

        let index = y * self.size.width + x;
        
        self.contents[index] = content.clone().into();
    }

    pub fn draw_rect<C: Clone>(&mut self, bounds: &Rect, content: &C) where Content: From<C>  {
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

    pub fn clear_with<C: Clone>(&mut self, content: &C) where Content: From<C>  {
        self.draw_rect(&Rect::from_size(&self.size), content);
    }
}