use std::{fs::File, io::Write};

use clothes::{
    graphics::{canvas::Canvas, pixel::RGBPixel},
    layout::{
        alignment::VerticalAlignment,
        geometry::Shape,
        node::{Node, ShapeBehavior},
        size_resolution::{SizeCalculator, SizeResolver},
    },
    output::ppm_output::PpmOutput,
    rendering::canvas::Drawable,
};

struct Palette;

impl Palette {
    fn dark_grey() -> RGBPixel {
        RGBPixel::new(64, 64, 64)
    }

    fn light_grey() -> RGBPixel {
        RGBPixel::new(192, 192, 192)
    }

    fn grey() -> RGBPixel {
        RGBPixel::new(128, 128, 128)
    }
}

fn main() -> std::io::Result<()> {
    let layout = Node::HorizontalStack(
        VerticalAlignment::Center,
        2,
        vec![
            Node::Shape(
                Shape::Capsule,
                ShapeBehavior::Stroke(1),
                Palette::light_grey(),
            )
            .width(5),
            Node::Shape(Shape::Capsule, ShapeBehavior::Fill, Palette::dark_grey()).height(30),
        ],
    )
    .padding_all(2)
    .background(Palette::grey());

    let mut canvas = Canvas::create(100, 100);
    let sized = SizeCalculator::resolve_size(&layout, &canvas.bounds(), &mut ());
    let draw_commands = SizeResolver::resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);

    let mut file = File::create(".debug-output/capsules.ppm")?;
    file.write(canvas.output().as_bytes())?;

    Ok(())
}
