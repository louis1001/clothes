use clothes::{canvas::{Canvas, TwoBitPixel}, layout::geometry::Rect};

fn main() {
    let mut canvas: Canvas<TwoBitPixel> = Canvas::create(10, 5);

    canvas.clear_with(&TwoBitPixel::Zero);

    canvas.draw_rect(&Rect::new(1, 1, 8, 3), &TwoBitPixel::One);
}
