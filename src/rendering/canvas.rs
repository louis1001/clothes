use std::u8;

use super::DrawCommand;
use crate::{
    graphics::canvas::Canvas,
    layout::geometry::{CornerRadius, Rect, Shape, Size},
};

use tiny_skia;

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
                    unimplemented!(
                        "At the moment, text is being rendered as glyphs through BitMap."
                    );
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
                DrawCommand::StrokeShape(bounds, stroke_width, shape, content) => {
                    self.stroke_shape(bounds, stroke_width.clone(), shape, content.clone());
                }
                DrawCommand::FillShape(bounds, shape, content) => {
                    self.fill_shape(bounds, shape, content.clone());
                }
            }
        }
    }

    fn stroke_shape(
        &mut self,
        bounds: &Rect,
        stroke_width: usize,
        shape: &Shape,
        content: Self::Content,
    ) {
        if bounds.width == 0 || bounds.height == 0 {
            return;
        }

        match shape {
            Shape::Rectangle => {
                let top_line = Rect::new(bounds.x, bounds.y, bounds.width, stroke_width);
                let right_line = Rect::new(bounds.max_x() - stroke_width as i64, bounds.y, stroke_width, bounds.height);
                let bottom_line = Rect::new(bounds.x, bounds.max_y() - stroke_width as i64, bounds.width, stroke_width);
                let left_line = Rect::new(bounds.x, bounds.y, stroke_width, bounds.height);

                self.fill_rect(&top_line, &content);
                self.fill_rect(&right_line, &content);
                self.fill_rect(&bottom_line, &content);
                self.fill_rect(&left_line, &content);
            }
            Shape::RoundedRectangle(corner_radius) => {
                self.stroke_path(
                    bounds,
                    stroke_width,
                    self.rounded_rectangle_path(bounds, corner_radius),
                    &content,
                );
            }
            Shape::Ellipse => {
                self.stroke_path(bounds, stroke_width, self.ellipse_path(bounds), &content);
            }
            Shape::Capsule => {
                self.stroke_path(bounds, stroke_width, self.capsule_path(bounds), &content)
            }
        }
    }

    fn stroke_path(
        &mut self,
        bounds: &Rect,
        stroke_width: usize,
        path: tiny_skia::Path,
        content: &Self::Content,
    ) {
        // FIXME: stroke width is a little broken, because skia draws within the bounds provided,
        // but because the stroke goes on either side of the path, half of it ends up outside of the bounds.
        // maybe scaling the bounds by the corner radius amount, and then scaling the drawing back.
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(u8::MAX, u8::MAX, u8::MAX, u8::MAX);
        paint.anti_alias = false;

        let mut stroke = tiny_skia::Stroke::default();
        stroke.width = stroke_width as f32;
        stroke.line_cap = tiny_skia::LineCap::Round;

        let mut pixmap = tiny_skia::Pixmap::new(bounds.width as u32, bounds.height as u32).unwrap();
        pixmap.stroke_path(
            &path,
            &paint,
            &stroke,
            tiny_skia::Transform::identity(),
            None,
        );

        let pixels = pixmap.pixels();

        let x = bounds.x as usize;
        let y = bounds.y as usize;

        for i in 0..pixels.len() {
            let pixel = pixels[i];
            let color = pixel.demultiply();

            if color.red() == 0 && color.green() == 0 && color.blue() == 0 && color.alpha() == 0 {
                // Not filled in. Skip
                continue;
            }

            let dx = i % bounds.width as usize;
            let dy = i / bounds.width as usize;

            let x = x + dx;
            let y = y + dy;

            self.write_pixel(content, x, y);
        }
    }

    fn fill_shape(&mut self, bounds: &Rect, shape: &Shape, content: Self::Content) {
        if bounds.width == 0 || bounds.height == 0 {
            return;
        }

        match shape {
            Shape::Rectangle => {
                self.fill_rect(&bounds, &content);
            }
            Shape::RoundedRectangle(corner_radius) => {
                self.fill_path(
                    bounds,
                    self.rounded_rectangle_path(bounds, corner_radius),
                    &content,
                );
            }
            Shape::Ellipse => {
                self.fill_path(bounds, self.ellipse_path(bounds), &content);
            }
            Shape::Capsule => self.fill_path(bounds, self.capsule_path(bounds), &content),
        }
    }

    fn fill_path(&mut self, bounds: &Rect, path: tiny_skia::Path, content: &Self::Content) {
        let mut paint = tiny_skia::Paint::default();
        paint.set_color_rgba8(u8::MAX, u8::MAX, u8::MAX, u8::MAX);
        let mut pixmap = tiny_skia::Pixmap::new(bounds.width as u32, bounds.height as u32).unwrap();
        pixmap.fill_path(
            &path,
            &paint,
            tiny_skia::FillRule::Winding,
            tiny_skia::Transform::identity(),
            None,
        );

        let pixels = pixmap.pixels();

        let x = bounds.x as usize;
        let y = bounds.y as usize;

        for i in 0..pixels.len() {
            let pixel = pixels[i];
            let color = pixel.demultiply();

            if color.red() == 0 && color.green() == 0 && color.blue() == 0 && color.alpha() == 0 {
                // Not filled in. Skip
                continue;
            }

            let dx = i % bounds.width as usize;
            let dy = i / bounds.width as usize;

            let x = x + dx;
            let y = y + dy;

            self.write_pixel(content, x, y);
        }
    }

    fn ellipse_path(&self, bounds: &Rect) -> tiny_skia::Path {
        let skia_bounds =
            tiny_skia::Rect::from_xywh(0.0, 0.0, bounds.width as f32, bounds.height as f32)
                .expect("Bounds should be all valid at this point");

        let mut path_builder = tiny_skia::PathBuilder::new();
        path_builder.push_oval(skia_bounds);

        path_builder.finish().expect("Path should build")
    }

    fn rounded_rectangle_path(&self, bounds: &Rect, corners: &CornerRadius) -> tiny_skia::Path {
        let skia_bounds =
            tiny_skia::Rect::from_xywh(0.0, 0.0, bounds.width as f32, bounds.height as f32)
                .expect("Bounds should be all valid at this point");

        // There's some jankiness correction needed, because when reaching the right or bottom edge,
        // the pixel get drawn outside of the bounds.
        let jankiness_correction = 0.5;

        let mut path_builder = tiny_skia::PathBuilder::new();

        let initial_x = skia_bounds.x() + corners.top_left as f32;
        let initial_y = 0.0 + jankiness_correction;

        path_builder.move_to(initial_x as f32, initial_y as f32);

        let top_right_start_x = skia_bounds.width() - corners.top_right as f32;

        path_builder.line_to(top_right_start_x, skia_bounds.top());

        if corners.top_right != 0 {
            let top_right_control_x = skia_bounds.width();
            let top_right_control_y = skia_bounds.top() + jankiness_correction;

            let top_right_end_x = skia_bounds.width() - jankiness_correction;
            let top_right_end_y = skia_bounds.top() + corners.top_right as f32;

            path_builder.quad_to(
                top_right_control_x,
                top_right_control_y,
                top_right_end_x,
                top_right_end_y,
            );
        }

        let bottom_right_start_y = skia_bounds.height() - corners.bottom_right as f32;
        let bottom_right_start_x = skia_bounds.right() - jankiness_correction;

        path_builder.line_to(bottom_right_start_x, bottom_right_start_y);

        if corners.bottom_right != 0 {
            let bottom_right_control_x = skia_bounds.width();
            let bottom_right_control_y = skia_bounds.height();

            let bottom_right_end_x = skia_bounds.width() - corners.bottom_right as f32;
            let bottom_right_end_y = skia_bounds.height() - jankiness_correction;

            path_builder.quad_to(
                bottom_right_control_x,
                bottom_right_control_y,
                bottom_right_end_x,
                bottom_right_end_y,
            );
        }

        let bottom_left_start_y = skia_bounds.height() - jankiness_correction;
        let bottom_left_start_x = skia_bounds.left() + corners.bottom_left as f32;

        path_builder.line_to(bottom_left_start_x, bottom_left_start_y);

        if corners.bottom_left != 0 {
            let bottom_left_control_x = skia_bounds.left() + jankiness_correction;
            let bottom_left_control_y = skia_bounds.height();

            let bottom_left_end_x = skia_bounds.left() + jankiness_correction;
            let bottom_left_end_y = skia_bounds.bottom() - corners.bottom_left as f32;

            path_builder.quad_to(
                bottom_left_control_x,
                bottom_left_control_y,
                bottom_left_end_x,
                bottom_left_end_y,
            );
        }

        let top_left_start_x = skia_bounds.left() + jankiness_correction;
        let top_left_start_y = skia_bounds.top() + corners.top_left as f32;

        path_builder.line_to(top_left_start_x, top_left_start_y);

        if corners.top_left != 0 {
            let top_left_control_x = skia_bounds.left() + jankiness_correction;
            let top_left_control_y = skia_bounds.top();

            let top_left_end_x = skia_bounds.left() + corners.top_left as f32;
            let top_left_end_y = skia_bounds.top() + jankiness_correction;

            path_builder.quad_to(
                top_left_control_x,
                top_left_control_y,
                top_left_end_x,
                top_left_end_y,
            );
        }

        path_builder.finish().expect("Path building should work.")
    }

    fn capsule_path(&self, bounds: &Rect) -> tiny_skia::Path {
        let short_length = bounds.width.min(bounds.height);
        let corner_radius = 1 + short_length / 2;

        self.rounded_rectangle_path(bounds, &CornerRadius::all(corner_radius))
    }
}

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
