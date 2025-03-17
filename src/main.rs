use std::io::Write;

use clothes::{
    graphics::{canvas::Canvas, pixel::TwoBitPixel},
    layout::{node::Node, size_resolution::{SizeCalculator, SizeResolver}},
    output::ppm_output::PpmOutput,
};

// FIXME: This breaks the vertical sizing, causing the UI to clip out of the bottom.
// Also, whitespace around the 
fn main() -> std::io::Result<()> {
    let mut canvas: Canvas<TwoBitPixel> = Canvas::create(200, 100);
    
    let ly = Node::text("
10 Home
20 sweet
30 goto 10
")
        .padding(1) 
        .background(true)
        .padding(3)
        .center()
        .background(true)
        .padding_horizontal(5);

    let sized = SizeCalculator::resolve_size(&ly, &canvas.bounds(), &mut ());
    let draw_commands = SizeResolver::resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);
    
    let image = canvas.output();

    let mut file = std::fs::File::create("./.debug-output/img.pgm")?;
    file.write_all(image.as_bytes())?;

    Ok(())
}
