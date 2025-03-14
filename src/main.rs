use std::io::Write;

use clothes::{
    graphics::{canvas::Canvas, pixel::TwoBitPixel},
    layout::geometry::Rect,
    output::ppm_output::PpmOutput,
};

fn main() -> std::io::Result<()> {
    let mut canvas: Canvas<TwoBitPixel> = Canvas::create(100, 100);

    canvas.clear_with(&TwoBitPixel::default());
    canvas.draw_rect(&Rect::new(40, 40, 20, 20), &TwoBitPixel::One);

    let image = canvas.output();
    let mut file = std::fs::File::create("./output.pgm")?;

    file.write_all(image.as_bytes())?;

    Ok(())
}
