use super::DrawCommand;
use crate::{graphics::canvas::Canvas, layout::geometry::{Rect, Size}};

pub trait Drawable {
    type Content: Clone + Default;
    fn fill_rect(&mut self, bounds: &Rect, content: &Self::Content);
    fn write_pixel(&mut self, content: &Self::Content, x: usize, y: usize);
    fn bounds(&self) -> Rect;

    fn size(&self) -> Size {
        self.bounds().size()
    }

    fn execute_draw_commands(&mut self, commands: &[DrawCommand<Self::Content>]) {
        for command in commands {
            match command {
                DrawCommand::TextLine(_, _, _) => {
                    unimplemented!("At the moment, text is being rendered as glyphs through BitMap.");
                }
                DrawCommand::FillRect(bounds, content) => {
                    self.fill_rect(bounds, content);
                }
                DrawCommand::StrokeRect(bounds, n, content) => {
                    // Top
                    for x in bounds.x..(bounds.x + bounds.width as i64) {
                        if x < 0 || x >= self.size().width as i64 {
                            continue;
                        }

                        for y in 0..*n {
                            let y_point = bounds.y + y as i64;
                            if y_point < 0 || y_point >= self.size().height as i64 {
                                continue;
                            }
                            self.write_pixel(content, x as usize, y_point as usize);
                        }
                    }

                    // Bottom
                    for x in bounds.x..(bounds.x + bounds.width as i64) {
                        if x < 0 || x >= self.size().width as i64 {
                            continue;
                        }

                        for y in 0..*n {
                            let y_point = bounds.y + bounds.height as i64 - y as i64 - 1;
                            if y_point < 0 || y_point >= self.size().height as i64 {
                                continue;
                            }
                            self.write_pixel(content, x as usize, y_point as usize);
                        }
                    }

                    // Left
                    for y in bounds.y..(bounds.y + bounds.height as i64) {
                        if y < 0 || y >= self.size().height as i64 {
                            continue;
                        }

                        for x in 0..*n {
                            let x_point = bounds.x + x as i64;
                            if x_point < 0 || x_point >= self.size().width as i64 {
                                continue;
                            }
                            self.write_pixel(content, x_point as usize, y as usize);
                        }
                    }

                    // Right
                    for y in bounds.y..(bounds.y + bounds.height as i64) {
                        if y < 0 || y >= self.size().height as i64 {
                            continue;
                        }

                        for x in 0..*n {
                            let x_point = bounds.x + bounds.width as i64 - x as i64 - 1;
                            if x_point < 0 || x_point >= self.size().width as i64 {
                                continue;
                            }
                            self.write_pixel(content, x_point as usize, y as usize);
                        }
                    }
                }
                DrawCommand::Bitmap(bitmap, bounds) => {
                    assert_eq!(
                        bitmap.len(),
                        bounds.width * bounds.height,
                        "Bitmap command dimensions don't match the map"
                    );

                    for dy in 0..(bounds.height) {
                        for dx in 0..(bounds.width) {
                            let x = dx + bounds.x as usize;
                            let y = dy + bounds.y as usize;

                            let Some(Some(content)) = bitmap.get(dy * bounds.width + dx) else {
                                continue;
                            };

                            self.write_pixel(content, x, y);
                        }
                    }
                }
            }
        }
    }
}

// impl<Content: Clone + Default> Canvas<Content> {
    
// }

impl<Content: Clone + Default> Drawable for Canvas<Content> {
    type Content = Content;

    fn fill_rect(&mut self, bounds: &Rect, content: &Self::Content) {
        self.draw_rect(bounds, content);
    }

    fn write_pixel(&mut self, content: &Self::Content, x: usize, y: usize) {
        self.write(content, x, y);
    }

    fn bounds(&self) -> Rect {
        Rect::from_size(&self.size)
    }
}