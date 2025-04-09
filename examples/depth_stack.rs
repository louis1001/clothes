// An example of use for the normal stack
use std::{fs::File, io::Write};

use clothes::{
    fonts::Font,
    graphics::{canvas::Canvas, pixel::RGBPixel},
    layout::{
        alignment::{Alignment, VerticalAlignment},
        geometry::{Matrix, Shape},
        node::{Node, ShapeBehavior},
        size_resolution::{SizeCalculator, SizeResolver},
    },
    output::ppm_output::PpmOutput,
    rendering::canvas::Drawable,
};

fn main() -> std::io::Result<()> {
    let canvas_sz = 100;
    let mut canvas = Canvas::create(canvas_sz, canvas_sz);

    let square = |sz, color, stroke: Option<usize>| {
        let shape = Node::Shape(
            Shape::Rectangle,
            stroke
                .map(|x| ShapeBehavior::Stroke(x))
                .unwrap_or(ShapeBehavior::Fill),
            color,
        );

        if let Some(sz) = sz {
            shape.width(sz).height(sz)
        } else {
            shape
        }
    };

    let example = |alignment: &Alignment| {
        Node::NormalStack(
            alignment.clone(),
            vec![
                square(None, RGBPixel::red(), Some(1)),
                square(Some(canvas_sz / 10), RGBPixel::green(), None),
            ],
        )
        .as_overlay(|| {
            Node::text_with_font(
                alignment_to_text(alignment).as_str(),
                RGBPixel::blue(),
                Font::four_by_five(),
            )
        })
    };

    let layout = Node::grid::<(), _, _>(
        &Matrix::with_rows(
            &vec![
                Alignment::top_left(),
                Alignment::top(),
                Alignment::top_right(),
                Alignment::left(),
                Alignment::center(),
                Alignment::right(),
                Alignment::bottom_left(),
                Alignment::bottom(),
                Alignment::bottom_right(),
            ],
            3,
        ),
        2,
        example,
    )
    .padding_all(2)
    .background(RGBPixel::white());

    let sized = SizeCalculator::resolve_size(&layout, &canvas.bounds(), &mut ());
    let draw_commands = SizeResolver::resolve_draw_commands(&sized, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);

    let mut output = File::create(".debug-output/depth_stack.ppm")?;

    output.write(canvas.output().as_bytes())?;

    Ok(())
}

fn alignment_to_text(alignment: &Alignment) -> String {
    let vertical = match alignment.vertical() {
        VerticalAlignment::Top => 'T',
        VerticalAlignment::Center => 'C',
        VerticalAlignment::Bottom => 'B',
    };

    let horizontal = match alignment.horizontal() {
        clothes::layout::alignment::HorizontalAlignment::Left => 'L',
        clothes::layout::alignment::HorizontalAlignment::Center => 'C',
        clothes::layout::alignment::HorizontalAlignment::Right => 'R',
    };

    format!("{vertical}{horizontal}")
}
