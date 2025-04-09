use std::{fmt::Debug, io::Write};

use crate::layout::alignment::VerticalAlignment;
use crate::output::ppm_output::PpmOutput;
use crate::{
    fonts::Font,
    graphics::{canvas::Canvas, pixel::TwoBitPixel},
    layout::{
        alignment::HorizontalAlignment,
        size_resolution::{SizeCalculator, SizeResolver},
    },
    rendering::canvas::Drawable,
    *,
};
use layout::node::Node;

#[test]
fn vertical_stack() {
    insta::assert_yaml_snapshot!(draw_layout::<TwoBitPixel>(
        "vertical_stack",
        Node::VerticalStack(
            HorizontalAlignment::Center,
            1,
            vec![
                dummy_component().width(50).background(TwoBitPixel(true)),
                dummy_component()
                    .height(20)
                    .width(4)
                    .background(TwoBitPixel(true))
                    .center_horizontally(),
                dummy_component()
                    .width(10)
                    .background(TwoBitPixel(true))
                    .center_vertically(),
                dummy_component()
                    .width(20)
                    .height(5)
                    .background(TwoBitPixel(true))
            ]
        )
        .background(TwoBitPixel(false))
    ));
}

#[test]
fn horizontal_stack() {
    insta::assert_yaml_snapshot!(draw_layout::<TwoBitPixel>(
        "horizontal_stack",
        Node::HorizontalStack(
            VerticalAlignment::Center,
            1,
            vec![
                dummy_component().width(50).background(TwoBitPixel(true)),
                dummy_component()
                    .center_vertically()
                    .width(4)
                    .background(TwoBitPixel(true)),
                dummy_component()
                    .height(10)
                    .background(TwoBitPixel(true))
                    .center_vertically(),
                dummy_component()
                    .center_horizontally()
                    .height(20)
                    .background(TwoBitPixel(true))
            ]
        )
        .background(TwoBitPixel(false))
    ));
}

fn draw_layout<Content: Debug + Default + Clone>(name: &str, node: Node<Content, ()>) -> String
where
    Canvas<Content>: PpmOutput,
{
    let mut canvas = Canvas::create(100, 100);

    let sized_nodes = SizeCalculator::resolve_size(&node, &canvas.bounds(), &mut ());
    let draw_commands = SizeResolver::resolve_draw_commands(&sized_nodes, &canvas.bounds());

    canvas.execute_draw_commands(&draw_commands);

    let output = canvas.output();

    write_example_output(name, &output);

    output
}

fn write_example_output(name: &str, contents: &String) {
    use std::fs::File;

    let _ = std::fs::create_dir("test_outputs");

    let Ok(mut file_output) = File::create(format!("test_outputs/{name}.ppm")) else {
        return;
    };

    let _ = file_output.write(contents.as_bytes());
}

fn dummy_component<Content: Debug + Default + Clone>() -> Node<Content, ()> {
    Node::Text("".to_string(), Font::three_by_three(), Content::default())
        .height(1)
        .width(1)
}
