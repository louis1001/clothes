pub mod canvas;

use crate::layout::geometry::Rect;

// FIXME: Should the draw commands contain the content (pixel value or color)?
// or can they be an environment value, to save on some memory?
// A draw command that sets and resets current content would be nice.
pub enum DrawCommand<Content: Clone + Default> {
    TextLine(Rect, String, Content),
    FillRect(Rect, Content),
    StrokeRect(Rect, usize, Content),
    Bitmap(Vec<Option<Content>>, Rect)
}