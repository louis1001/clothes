use std::io::Write;

use clothes::{
    graphics::{canvas::Canvas, pixel::RGBPixel}, layout::{alignment::Edge, node::Node, size_resolution::{SizeCalculator, SizeResolver}}, output::ppm_output::PpmOutput
};

fn main() -> std::io::Result<()> {
    let mut canvas: Canvas<RGBPixel> = Canvas::create(100, 100);
    
    let ly = Node::text("Pixel\nPerfect")
        .padding(2)
        .width(35)
        .height(35)
        .border(1, RGBPixel::blue(), Edge::all())
        .center()
        .background(RGBPixel::white());

    let sized = SizeCalculator::resolve_size(&ly, &canvas.bounds(), &mut ());

    println!("{:#?}", sized);
    let draw_commands = SizeResolver::resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);

    let image = canvas.output();

    let mut file = std::fs::File::create("./.debug-output/img.pgm")?;
    file.write_all(image.as_bytes())?;

    Ok(())
}
