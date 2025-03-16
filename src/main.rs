use std::io::Write;

use clothes::{
    graphics::{canvas::Canvas, pixel::TwoBitPixel},
    layout::geometry::Rect,
    output::ppm_output::PpmOutput, rendering::DrawCommand,
};

fn main() -> std::io::Result<()> {
    let canvas_size = 100;
    let mut canvas= Canvas::create(canvas_size, canvas_size);
    
    let square_size = 35;

    let square_origin_y = (canvas_size/2 - square_size/2) as i64;
    let square_origin_x = square_origin_y - 5;

    let border_width = 1;

    let commands = vec![
        DrawCommand::StrokeRect(
            Rect::new(
                square_origin_x,
                square_origin_y,
                square_size, square_size
            ),
            border_width as usize, true.into()
        ),
        DrawCommand::TextLine(
            Rect::new(
                square_origin_x + (border_width+2),
                square_origin_y + (border_width+2),
                100, 3
            ),
            "\"Pixel perfect\"".to_string(),
            TwoBitPixel::One
        ),
        DrawCommand::FillRect(
            Rect::new(canvas_size as i64 - 31, canvas_size as i64 - 5, 31, 5),
            TwoBitPixel::One
        ),
        DrawCommand::TextLine(
            Rect::new(canvas_size as i64 - 30, canvas_size as i64 - 4, 30, 3),
            "08:06 PM".to_string(),
            TwoBitPixel::Zero
        )
    ];

    canvas.execute_draw_commands(&commands);

    let image = canvas.output();
    let mut file = std::fs::File::create(".debug-output/img.pgm")?;

    file.write_all(image.as_bytes())?;

    Ok(())
}
