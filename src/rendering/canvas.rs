use crate::{fonts::{ascii::Ascii, Font, Glyph}, graphics::canvas::Canvas, layout::geometry::{Rect, Size}};
use super::DrawCommand;

impl<Content: Clone + Default> Canvas<Content> {
    pub fn execute_draw_commands(&mut self, commands: &[DrawCommand<Content>]) {
        for command in commands {
            match command {
                DrawCommand::TextLine(bounds, text, content) => {
                    let mut extra_commands = vec![];
                    // FIXME: Hardcoding a type of text for now.
                    let font = Font::singleton();

                    let mut current_x = bounds.x;

                    let mut emit_draw_command = |glyph: &Glyph, size: &Size, x| {
                        extra_commands.push(
                            DrawCommand::Bitmap(
                                glyph.map(|b| {
                                    if b {
                                        Some(content.clone())
                                    } else {
                                        None
                                    }
                                }),
                                Rect::new(x, bounds.y, size.width, size.height)
                            )
                        );

                        x + size.width as i64 + font.character_spacing() as i64
                    };

                    for c in text.chars() {
                        let Ok(ascii) = Ascii::try_from(c) else {
                            current_x = emit_draw_command(font.default_glyph(), font.size(Ascii(b'0')), current_x);
                            continue;
                        };

                        match ascii {
                            Ascii(b' ') => current_x += font.space_width() as i64,
                            c => current_x = emit_draw_command(font.get_glyph(&c), font.size(c), current_x)
                        }
                    }

                    self.execute_draw_commands(&extra_commands);
                }
                DrawCommand::FillRect(bounds, content) => {
                    self.draw_rect(bounds, content);
                }
                DrawCommand::StrokeRect(bounds, n, content) => {
                    // Top
                    for x in bounds.x..(bounds.x + bounds.width as i64) {
                        if x < 0 || x >= self.size.width as i64 { continue; }
                        
                        for y in 0..*n {
                            let y_point = bounds.y + y as i64;
                            if y_point < 0 || y_point >= self.size.height as i64 { continue; }
                            self.write(content, x as usize, y_point as usize);
                        }
                    }
    
                    // Bottom
                    for x in bounds.x..(bounds.x + bounds.width as i64) {
                        if x < 0 || x >= self.size.width as i64 { continue; }
                        
                        for y in 0..*n {
                            let y_point = bounds.y + bounds.height as i64 - y as i64 - 1;
                            if y_point < 0 || y_point >= self.size.height as i64 { continue; }
                            self.write(content, x as usize, y_point as usize);
                        }
                    }
    
                    // Left
                    for y in bounds.y..(bounds.y + bounds.height as i64) {
                        if y < 0 || y >= self.size.height as i64 { continue; }
                        
                        for x in 0..*n {
                            let x_point = bounds.x + x as i64;
                            if x_point < 0 || x_point >= self.size.width as i64 { continue; }
                            self.write(content, x_point as usize, y as usize);
                        }
                    }
    
                    // Right
                    for y in bounds.y..(bounds.y + bounds.height as i64) {
                        if y < 0 || y >= self.size.height as i64 { continue; }
                        
                        for x in 0..*n {
                            let x_point = bounds.x + bounds.width as i64 - x as i64 - 1;
                            if x_point < 0 || x_point >= self.size.width as i64 { continue; }
                            self.write(content, x_point as usize, y as usize);
                        }
                    }
                }
                DrawCommand::Bitmap(bitmap, bounds) => {
                    assert_eq!(bitmap.len(), bounds.width * bounds.height, "Bitmap command dimensions don't match the map");

                    for dy in 0..(bounds.height) {
                        for dx in 0..(bounds.width) {
                            let x = dx + bounds.x as usize;
                            let y = dy + bounds.y as usize;

                            let Some(Some(content)) = bitmap.get(dy * bounds.height + dx) else {
                                continue;
                            };

                            self.write(content, x, y);
                        }
                    }
                }
            }
        }
    }
}