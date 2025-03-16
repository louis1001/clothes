use std::io::Write;

use clothes::{
    graphics::{canvas::Canvas, pixel::TwoBitPixel},
    layout::geometry::Rect,
    output::ppm_output::PpmOutput, rendering::DrawCommand,
};

fn main() -> std::io::Result<()> {
    let canvas_size = 100;
    let mut canvas= Canvas::create(canvas_size, canvas_size);
    
    use TwoBitPixel::*;
    
    let bitmap = vec![
        None, Some(One), None,
        Some(One), None, Some(One),
        None, Some(One), None,
    ];

    let commands = vec![
        DrawCommand::StrokeRect(
            Rect::new(
                20, 20,
                40, 40
            ),
            2, true.into()
        ),
        DrawCommand::Bitmap(bitmap, Rect::new(2, 2, 3, 3)),
        DrawCommand::TextLine(
            Rect::new(
                canvas_size as i64 - 60,
                canvas_size as i64 - 10,
                100, 5
            ),
            "hello, world...".to_string(),
            TwoBitPixel::One
        )
    ];

    canvas.execute_draw_commands(&commands);

    let image = canvas.output();
    let mut file = std::fs::File::create(".debug-output/img.pgm")?;

    file.write_all(image.as_bytes())?;

    Ok(())
}
