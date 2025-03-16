use std::io::Write;

use clothes::{
    graphics::{canvas::Canvas, pixel::TwoBitPixel},
    layout::{alignment::Edge, node::Node, size_resolution},
    output::ppm_output::PpmOutput,
};

fn main() -> std::io::Result<()> {
    let mut canvas: Canvas<TwoBitPixel> = Canvas::create(100, 100);
    
    let ly = Node::vertical_stack(vec![
        Node::text("Hey").center_horizontally(),
        Node::text("How is it going?").padding_top(3)
        .center_horizontally(),
    ]).center().background(true.into()).border(1, false.into(), Edge::all());

    println!("{:#?}", ly);

    let sizer = size_resolution::SizeCalculator::new();
    let resolver = size_resolution::SizeResolver::new();

    let sized = sizer.resolve_size(&ly, &canvas.bounds(), &mut ());
    let draw_commands = resolver.resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);
    
    let image = canvas.output();

    let mut file = std::fs::File::create("./.debug-output/img.pgm")?;
    file.write_all(image.as_bytes())?;

    Ok(())
}
