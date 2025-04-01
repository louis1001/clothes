use std::{fs::File, io::Write};

use clothes::{dictionary, graphics::{canvas::Canvas, pixel::RGBPixel}, layout::{alignment::{Edge, HorizontalAlignment, VerticalAlignment}, node::Node, size_resolution::{SizeCalculator, SizeResolver}}, output::ppm_output::PpmOutput, rendering::canvas::Drawable};

extern crate clothes;

fn main() -> std::io::Result<()> {
    let mut canvas = Canvas::create(100, 100);

    let layout = Node::HorizontalStack(VerticalAlignment::Center, 1, vec![
        Node::text(" ", RGBPixel::red()).width(5).height(5).background(RGBPixel::red()),
        Node::text(" ", RGBPixel::green()).width(5).height(5).background(RGBPixel::green()),
        Node::text(" ", RGBPixel::blue()).width(5).height(5).background(RGBPixel::blue()),
        Node::text(" ", RGBPixel::black()).width(5).height(5).background(RGBPixel::black()),
        Node::text(" ", RGBPixel::white()).width(5).height(5).background(RGBPixel::white()),
    ])
    .padding(1, Edge::all())
    .border(1, RGBPixel::blue(), Edge::all())
    .center()
    .background(RGBPixel::white());

    let sized = SizeCalculator::resolve_size(&layout, &canvas.bounds(), &mut ());
    let draw_commands = SizeResolver::resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);

    let mut output = File::create(".debug-output/simple.ppm")?;

    output.write(canvas.output().as_bytes())?;

    Ok(())
}