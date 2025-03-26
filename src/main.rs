use std::io::Write;

use clothes::{
    graphics::{canvas::Canvas, pixel::RGBPixel},
    layout::{
        node::Node,
        size_resolution::{SizeCalculator, SizeResolver},
    },
    output::ppm_output::PpmOutput,
};

fn main() -> std::io::Result<()> {
    let mut canvas: Canvas<RGBPixel> = Canvas::create(160, 160);

    let ly = app();

    let sized = SizeCalculator::resolve_size(&ly, &canvas.bounds(), &mut ());

    let draw_commands = SizeResolver::resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);

    let image = canvas.output();

    let mut file = std::fs::File::create("./.debug-output/img.pgm")?;
    file.write_all(image.as_bytes())?;

    Ok(())
}

fn app<Ctx: Clone + std::fmt::Debug + Default>(
) -> Node<RGBPixel, Ctx> {
    Node::text(include_str!("./main.rs"), 
    RGBPixel::green())
    .padding_all(1)
}
