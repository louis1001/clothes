# Clothes

A layout engine inspired by SwiftUI and declarative UI frameworks. Based on the foundations build on my other project, [https://github.com/louis1001/textyle](Textyle), which was mainly focused on text and terminal based UI.

> This readme is a **work in progress**. I'll add to it when I've prepared enough examples •u•

## Basic usage
To use this library, you need to grasp a few basic components:
- **Layout node**: A component that describes a visual element or layout component.
- **Canvas**: A collection of *content* (usually pixels) that represents the drawing environment. This can be anything, as long as it implements the `Drawable` trait.
- **Draw Commands**: Instructions on what to draw and where.

The default program loop is:

```rust
// 1. Define the layout
let layout = Node::text("Hello").padding_all();
// Any layout. Nodes implement some modifiers that make this easier.

// 2. Create the drawing environment. Usually, a Canvas.
let mut canvas = Canvas::create(200, 200);

// 3. Transform the layout node into a series of draw commands.
// for this there are a couple of steps:

//   a. Calculate approximate sizes for nodes.
let sized_nodes = SizeCalculator::resolve_size(&layout, &canvas.bounds(), &mut ());

//   b. Resolve the approximations to real bounds, then convert to draw commands.
let draw_commands = SizeResolver::resolve_draw_commands(&sized_nodes, &canvas.bounds());

// 4. Lastly, execute the draw commands on the drawing environment.
canvas.execute_draw_commands(&draw_commands);
```

## Example results
Here are some examples of layout trees, and their results after rendering.

### A normal stack (or ZStack)
<details>

```rust
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
            RGBPixel::white(),
            Font::four_by_five(),
        )
    })
};

let layout = Node::VerticalStack(
    HorizontalAlignment::Center,
    1,
    vec![
        Node::text("Normal Stack", RGBPixel::white()).padding_top(1),
        Node::grid::<(), _, _>(
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
        .background(RGBPixel::black()),
    ],
);
```

<summary>The Layout Code</sumary>
</details>

![NormalStack example](https://github.com/louis1001/clothes/blob/master/assets/depth_stack_example.jpg?raw=true)

### Shapes

<details>

```rust
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
```

<summary>The Layout Code</sumary>
</details>

![Shapes example](https://github.com/louis1001/clothes/blob/master/assets/shapes_example.jpg?raw=true)

### VerticalStack

<details>

```rust
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
```

<summary>The Layout Code</sumary>
</details>

![Swatches example](https://github.com/louis1001/clothes/blob/master/assets/simple_example.jpg?raw=true)
