use clothes::{graphics::{canvas::Canvas, pixel::{RGBPixel, TwoBitPixel}}, layout::geometry::Rect};

fn main() {
    let mut canvas: Canvas<RGBPixel> = Canvas::create(10, 5);

    canvas.clear_with(&RGBPixel::default());

    canvas.draw_rect(&Rect::new(1, 1, 8, 3), &TwoBitPixel::One);
}
