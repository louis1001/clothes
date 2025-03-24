use std::io::Write;

use clothes::{
    dictionary, graphics::{canvas::Canvas, pixel::RGBPixel}, layout::{alignment::{Edge, HorizontalAlignment, VerticalAlignment}, node::Node, size_resolution::{SizeCalculator, SizeResolver}}, output::ppm_output::PpmOutput
};

fn main() -> std::io::Result<()> {
    let mut canvas: Canvas<RGBPixel> = Canvas::create(200, 50);
    
    let ly = Node::HorizontalStack(VerticalAlignment::Top, 0, vec![
        Node::text("Main content")
            .center_horizontally()
            .align_top()
            .padding_vertical(2)
            .border(2, RGBPixel::red(), dictionary!(Edge::Right)),
        Node::VerticalStack(HorizontalAlignment::Center, 0, vec![
            Node::text_with_content("Side content", RGBPixel::blue())
            .background(RGBPixel::green()),
            Node::VerticalStack(HorizontalAlignment::Left, 1, vec![
                Node::text("List of content:")
                .padding(1),
                Node::text("- Item 1"),
                Node::text("- Item 2"),
                Node::text("- Item 3"),
            ])
            .padding_horizontal(5)
            .padding_top(2)
            .border(1, RGBPixel::green(), dictionary![Edge::Top])
        ])
        .padding_vertical(2)
    ]).background(RGBPixel::white());

    let sized = SizeCalculator::resolve_size(&ly, &canvas.bounds(), &mut ());

    let draw_commands = SizeResolver::resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);

    let image = canvas.output();

    let mut file = std::fs::File::create("./.debug-output/img.pgm")?;
    file.write_all(image.as_bytes())?;

    Ok(())
}
