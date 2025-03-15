mod canvas;

use crate::layout::geometry::Rect;

pub enum DrawCommand<Content: Clone + Default> {
    Text(Rect, String),
    FillRect(Rect, Content),
    StrokeRect(Rect, usize, Content),
}