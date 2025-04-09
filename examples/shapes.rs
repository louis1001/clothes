use std::{
    fs::File,
    io::{Result, Write},
};

use clothes::{
    graphics::{canvas::Canvas, pixel::RGBPixel},
    layout::{
        alignment::Alignment, geometry::{CornerRadius, Shape}, node::{Node, ShapeBehavior}, size_resolution::{SizeCalculator, SizeResolver}
    },
    output::ppm_output::PpmOutput,
    rendering::canvas::Drawable,
};

extern crate clothes;

fn main() -> Result<()> {
    let layout = Node::vertical_stack(vec![
        Node::plain_text("Hey!"),
        Node::NormalStack(Alignment::top(), vec![
            Node::Shape(
                Shape::RoundedRectangle(CornerRadius::new(5, 5, 5, 20)),
                ShapeBehavior::Fill,
                RGBPixel::blue(),
            )
            .padding_all(1),
            Node::vertical_stack(vec![
                Node::Shape(Shape::Ellipse, ShapeBehavior::Stroke(1), RGBPixel::green()),
                Node::Shape(Shape::Ellipse, ShapeBehavior::Fill, RGBPixel::black()),
            ]).padding_horizontal(6).padding_vertical(3),
        ])
        .width(60)
        .center(),
    ])
    .padding_all(2)
    .background(RGBPixel::red());

    let mut canvas = Canvas::create(100, 100);
    let sized = SizeCalculator::resolve_size(&layout, &canvas.bounds(), &mut ());
    let draw_commands = SizeResolver::resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);

    let mut file = File::create(".debug-output/shapes.ppm")?;
    file.write(canvas.output().as_bytes())?;

    Ok(())
}
