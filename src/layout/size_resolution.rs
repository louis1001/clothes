use std::fmt::Debug;

use crate::{fonts::{ascii::Ascii, Font}, graphics::pixel::TwoBitPixel, layout::{self, alignment, sizing}, rendering::DrawCommand};

use super::{geometry::{Rect, Size}, node::Node, sized_node::{SizedNode, SizedItem}};

// Calculate size
pub struct SizeCalculator<Content: Clone + Default, Ctx: Clone> {
    _content_marker: std::marker::PhantomData<Content>,
    _marker: std::marker::PhantomData<Ctx>,
}

impl<Content: Clone + Default, Ctx: Clone> SizeCalculator<Content, Ctx> {
    pub fn new() -> Self {
        SizeCalculator {
            _content_marker: std::marker::PhantomData{},
            _marker: std::marker::PhantomData{}
        }
    }
}

impl<Content: Clone + Default + Debug, Ctx: Clone + std::fmt::Debug> SizeCalculator<Content, Ctx> {
    fn calculate_line_size(&self, line: &str, bounds: &Rect, font: &Font) -> Size {
        let ascii_elements = line
            .chars()
            .filter_map(|x| Ascii::try_from(x).ok());

        let mut row_count = 1;
        let mut max_width: usize = 0;
        let mut current_width = 0;
        let mut character_height = 0;

        for element in ascii_elements {
            if element.0 == b' ' {
                // FIXME: Add character spacing before space?
                current_width += font.space_width();
                continue;
            }

            let element_size = font.size(element);

            if character_height == 0 {
                // FIXME: Better height calculation
                character_height = element_size.height;
            }

            let new_width = current_width + font.character_spacing() + element_size.width;

            if new_width > bounds.width {
                current_width = 0;
                row_count += 1;

                continue;
            }

            current_width = new_width;
            max_width = current_width;
        }

        let height = row_count * character_height + (row_count - 1 * font.line_spacing());

        Size::new(max_width, height)
    }

    pub fn resolve_size(&self, container_node: &Node<Content, Ctx>, bounds: &Rect, context: &mut Ctx) -> SizedNode<Content> {
        use Node::*;
        use sizing::Sizing::*;

        match container_node {
            Text(t) => {
                let font = Font::singleton();
                let lines = t.lines();

                let mut width = 0usize;
                let mut height = 0usize;
                for line in lines {
                    let sz = self.calculate_line_size(line, bounds, font);
                    if sz.width > width {
                        width = sz.width;
                    }

                    height += sz.height;
                }

                // TODO: Introduce `Flexible` item sizing to handle better text sizing.
                let sizing = sizing::ItemSizing::new(Static(width), Static(height));

                SizedNode::new(SizedItem::Text(t.clone()), sizing)
            }
            VCenter(node) => {
                let resolved = self.resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_height = content_size.vertical.min_content_size();

                let sizing = sizing::ItemSizing::new(content_size.horizontal, Greedy(min_height));

                SizedNode::new(SizedItem::VCenter(resolved), sizing)
            }
            VBottomAlign(node) => {
                let resolved = self.resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_height = content_size.vertical.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: content_size.horizontal, vertical: Greedy(min_height) };

                SizedNode::new(SizedItem::VBottomAlign(resolved), sizing)
            }
            HCenter(node) => {
                let resolved = self.resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_width = content_size.horizontal.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: Greedy(min_width), vertical: content_size.vertical };

                SizedNode::new(SizedItem::HCenter(resolved), sizing)
            }
            HRightAlign(node) => {
                let resolved = self.resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_width = content_size.horizontal.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: Greedy(min_width), vertical: content_size.vertical };

                SizedNode::new(SizedItem::HRightAlign(resolved), sizing)
            }
            VTopAlign(node) => {
                let resolved = self.resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_height = content_size.vertical.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: content_size.horizontal, vertical: Greedy(min_height) };

                SizedNode::new(SizedItem::VTopAlign(resolved), sizing)
            }
            HLeftAlign(node) => {
                let resolved = self.resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_width = content_size.horizontal.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: Greedy(min_width), vertical: content_size.vertical };

                SizedNode::new(SizedItem::HLeftAlign(resolved), sizing)
            }
            Width(size, node) => {
                let mut bounds = bounds.clone();
                bounds.width = *size;

                let resolved_content = self.resolve_size(node, &bounds, context);
                let mut frame = resolved_content.sizing.clone();
                frame.horizontal = Static(*size);

                SizedNode::new(SizedItem::Width(*size, resolved_content), frame)
            }
            Height(size, node) => {
                let mut bounds = bounds.clone();
                bounds.height = *size;

                let resolved_content = self.resolve_size(node, &bounds, context);
                let mut frame = resolved_content.sizing.clone();
                frame.vertical = Static(*size);

                SizedNode::new(SizedItem::Height(*size, resolved_content), frame)
            }
            TopPadding(n, node) | BottomPadding(n, node) => {
                let resolved = self.resolve_size(node, bounds, context);
                let mut frame = resolved.sizing.clone();
                
                frame.vertical.clamped_add(*n);

                let make_node = |n: usize, node: SizedNode<Content>|{
                    match container_node {
                        TopPadding(_, _) => SizedItem::TopPadding(n, node),
                        BottomPadding(_, _) => SizedItem::BottomPadding(n, node),
                        _ => unreachable!()
                    }
                };

                if frame.vertical.min_content_size() > bounds.height {
                    // recalculate with less space
                    let mut bounds = bounds.clone();
                    bounds.height = bounds.height.saturating_sub(*n);

                    let resolved_content = self.resolve_size(node, &bounds, context);
                    let mut frame = resolved_content.sizing.clone();

                    frame.vertical.clamped_add(*n);

                    SizedNode::new(make_node(*n, resolved_content), frame)
                } else {
                    SizedNode::new(make_node(*n, resolved), frame)
                }
            }
            LeftPadding(n, node) | RightPadding(n, node) => {
                let resolved = self.resolve_size(node, bounds, context);
                let mut frame = resolved.sizing.clone();

                let make_node = |n: usize, node: SizedNode<Content>|{
                    match container_node {
                        LeftPadding(_, _) => SizedItem::LeftPadding(n, node),
                        RightPadding(_, _) => SizedItem::RightPadding(n, node),
                        _ => unreachable!()
                    }
                };
                
                frame.horizontal.clamped_add(*n);
                if frame.horizontal.min_content_size() > bounds.width {
                    // recalculate with less space
                    let mut bounds = bounds.clone();
                    bounds.width = bounds.width.saturating_sub(*n);

                    let resolved_content = self.resolve_size(node, &bounds, context);
                    frame = resolved_content.sizing.clone();
                    frame.horizontal.clamped_add(*n);

                    let node = make_node(*n, resolved_content);

                    SizedNode::new(node, frame)
                } else {
                    SizedNode::new(make_node(*n, resolved), frame)
                }
            }
            Background(c, node) => {
                let resolved_content = self.resolve_size(node, bounds, context);
                let frame = resolved_content.sizing.clone();

                SizedNode::new(SizedItem::Background(c.clone(), resolved_content), frame)
            }
            Border(n, c, edges, node) => {
                let outer_bounds = bounds;
                let mut resolved_content = self.resolve_size(node, outer_bounds, context);
                let mut frame = resolved_content.sizing.clone();

                let mut added_height = 0;
                
                if edges.contains(&alignment::Edge::Top) {
                    frame.vertical.clamped_add(*n);
                    added_height += *n;
                }
                if edges.contains(&alignment::Edge::Bottom) {
                    frame.vertical.clamped_add(*n);
                    added_height += *n;
                }

                if frame.vertical.min_content_size() > outer_bounds.height {
                    // recalculate with less space
                    let mut bounds = outer_bounds.clone();
                    bounds.height = bounds.height.saturating_sub(added_height);

                    resolved_content = self.resolve_size(node, &bounds, context);
                    frame = resolved_content.sizing.clone();

                    frame.vertical.clamped_add(added_height);
                }

                let mut added_width = 0;

                if edges.contains(&alignment::Edge::Left) {
                    frame.horizontal.clamped_add(*n);
                    added_width += *n;
                }
                if edges.contains(&alignment::Edge::Right) {
                    frame.horizontal.clamped_add(*n);
                    added_width += *n;
                }

                if frame.horizontal.min_content_size() > outer_bounds.width {
                    // recalculate with less space
                    let mut bounds = outer_bounds.clone();
                    bounds.width = bounds.width.saturating_sub(added_width);

                    resolved_content = self.resolve_size(node, &bounds, context);
                    frame = resolved_content.sizing.clone();

                    frame.horizontal.clamped_add(added_width);
                }

                SizedNode::new(SizedItem::Border(*n, c.clone(), edges.clone(), resolved_content), frame)
            }

            VerticalStack(alignment, spacing,  nodes) => {
                let spacing_sizing = spacing * nodes.len().saturating_sub(1);
                let mut result = sizing::ItemSizing { horizontal: Static(0), vertical: Static(spacing_sizing) };
                let mut bounds = bounds.clone();
                bounds.height = bounds.height.saturating_sub(spacing_sizing);
                let mut resolved_children: Vec<SizedNode<_>> = vec![];

                for node in nodes {
                    let resolved_node = self.resolve_size(node, &bounds, context);
                    let node_sizing = resolved_node.sizing.clone();
                    result.horizontal = match result.horizontal {
                        Static(j) => match node_sizing.horizontal {
                            Static(i) => Static(i.max(j)),
                            Greedy(i) => Greedy(i.max(j))
                        }
                        Greedy(j) => {
                            let i = node_sizing.horizontal.min_content_size();
                            Greedy(i.max(j))
                        }
                    };

                    result.vertical.clamped_accumulate_constrained(&node_sizing.vertical, bounds.height);
                    resolved_children.push(resolved_node);
                }

                SizedNode::new(SizedItem::VerticalStack(alignment.clone(), *spacing, resolved_children), result)
            }
            HorizontalStack(alignment, spacing, nodes) => {
                let spacing_sizing = spacing * nodes.len().saturating_sub(1);
                let mut result = sizing::ItemSizing { horizontal: Static(spacing_sizing), vertical: Static(0) };let mut bounds = bounds.clone();
                bounds.width -= spacing_sizing;

                let mut resolved_children = vec![];

                for node in nodes {
                    let resolved_node = self.resolve_size(node, &bounds, context);
                    let node_sizing = resolved_node.sizing.clone();
                    result.vertical = match result.vertical {
                        Static(j) => match node_sizing.vertical {
                            Static(i) => Static(i.max(j)),
                            Greedy(i) => Greedy(i.max(j))
                        }
                        Greedy(j) => {
                            let i = node_sizing.vertical.min_content_size();
                            Greedy(i.max(j))
                        }
                    };

                    result.horizontal.clamped_accumulate_constrained(&node_sizing.horizontal, bounds.width);

                    resolved_children.push(resolved_node);
                }

                SizedNode::new(SizedItem::HorizontalStack(alignment.clone(), *spacing, resolved_children), result)
            }
            WithContext(node) => {
                let node = node(context);

                self.resolve_size(&node, bounds, context)
            }
        }
    }
}

// Calculate size
pub struct SizeResolver<Content: Clone + Default> {
    _marker: std::marker::PhantomData<Content>
}

impl<Content: Clone + Default> SizeResolver<Content> {
    pub fn new() -> Self {
        Self{
            _marker: std::marker::PhantomData {}
        }
    }
}

impl<Content: Clone + Default> SizeResolver<Content> {
    pub fn resolve_draw_commands(&self, sized_node: &SizedNode<Content>, bounds: &Rect) -> Vec<DrawCommand<Content>> {
        use SizedItem::*;
        let layout = sized_node.clone();

        match *layout.node {
            Text(text) => {
                // TODO: Handle current content (foreground, background style)

                // TODO: Convert text into text lines (split by new line, handle when a line wraps...)
                vec![DrawCommand::TextLine(bounds.clone(), text, Content::default())]
            }
            Width(_, node) | Height(_, node) => {
                let frame = node.sizing.fit_into(bounds);

                self.resolve_draw_commands(&node, &frame)
            }
            VCenter(n) => {
                let mut content_rect = n.sizing.fit_into(bounds);
                let center_pos = bounds.y as usize + bounds.height / 2;
                let center_start = center_pos - content_rect.height / 2;
                content_rect.y = center_start as i64;

                let content_bounds = n.sizing.fit_into(&content_rect);

                self.resolve_draw_commands(&n, &content_bounds)
            }
            HCenter(n) => {
                let mut content_rect = n.sizing.fit_into(bounds);
                let center_pos = bounds.x as usize + bounds.width / 2;
                let center_start = center_pos - content_rect.width / 2;
                content_rect.x = center_start as i64;

                let content_bounds = n.sizing.fit_into(&content_rect);

                self.resolve_draw_commands(&n, &content_bounds)
            }
            VBottomAlign(n) => {
                let mut content_rect = n.sizing.fit_into(bounds);
                let bottom_most = bounds.y as usize + bounds.height;
                let top_start = bottom_most - content_rect.height;
                content_rect.y = top_start as i64;

                self.resolve_draw_commands(&n, &content_rect)
            }
            HRightAlign(n) => {
                let mut content_rect = n.sizing.fit_into(bounds);
                let right_most = bounds.x as usize + bounds.width;
                let left_start = right_most - content_rect.width;
                content_rect.x = left_start as i64;

                let content_bounds = n.sizing.fit_into(&content_rect);

                self.resolve_draw_commands(&n, &content_bounds)
            }
            VTopAlign(n) | HLeftAlign(n) => {
                let content_rect = n.sizing.fit_into(bounds);

                self.resolve_draw_commands(&n, &content_rect)
            }
            TopPadding(n, node) => {
                let mut bounds = bounds.clone();
                bounds.height = bounds.height.saturating_sub(n);
                let mut frame = node.sizing.fit_into(&bounds);
                frame.x = bounds.x;
                frame.y = bounds.y + n as i64;

                self.resolve_draw_commands(&node, &frame)
            }
            BottomPadding(n, node) => {
                let mut bounds = bounds.clone();
                bounds.height = bounds.height.saturating_sub(n);

                let mut frame = node.sizing.fit_into(&bounds);
                frame.x = bounds.x;
                frame.y = bounds.y;

                self.resolve_draw_commands(&node, &frame)
            }
            RightPadding(n, node) => {
                let mut frame = node.sizing.fit_into(bounds);
                frame.x = bounds.x;
                frame.y = bounds.y;

                let free_width = bounds.width.saturating_sub(n);
                let adjustment = frame.width.saturating_sub(free_width);

                frame.width = frame.width.saturating_sub(adjustment);

                self.resolve_draw_commands(&node, &frame)
            }
            LeftPadding(n, node) => {
                let mut bounds = bounds.clone();
                bounds.width = bounds.width.saturating_sub(n);
                let mut frame = node.sizing.fit_into(&bounds);
                frame.x = bounds.x + n as i64;
                frame.y = bounds.y;

                self.resolve_draw_commands(&node, &frame)
            }
            Background(background_style, node) => {
                let mut frame = node.sizing.fit_into(bounds);
                frame.x = bounds.x;
                frame.y = bounds.y;

                let mut commands = vec![DrawCommand::FillRect(bounds.clone(), background_style)];

                let content_commands = self.resolve_draw_commands(&node, &frame);

                commands.extend(content_commands);

                commands
            }
            Border(n, border_style, edges, node) => {
                let outer_bounds = bounds;
                let mut inner_bounds = bounds.clone();
                for edge in &edges {
                    match edge {
                        layout::alignment::Edge::Top => {
                            inner_bounds.height = inner_bounds.height.saturating_sub(n);
                            inner_bounds.y = inner_bounds.y.checked_add(n as i64).unwrap_or(0);
                        }
                        layout::alignment::Edge::Right => {
                            inner_bounds.width = inner_bounds.width.saturating_sub(n);
                        }
                        layout::alignment::Edge::Bottom => {
                            inner_bounds.height = inner_bounds.height.saturating_sub(n);
                        }
                        layout::alignment::Edge::Left => {
                            inner_bounds.width = inner_bounds.width.saturating_sub(n);
                            inner_bounds.x = inner_bounds.x.checked_add(n as i64).unwrap_or(0);
                        }
                    }
                }

                let mut frame = node.sizing.fit_into(&inner_bounds);
                frame.x = inner_bounds.x;
                frame.y = inner_bounds.y;

                let mut commands = self.resolve_draw_commands(&node, &frame);

                if edges == layout::alignment::Edge::all() {
                    commands.push(DrawCommand::StrokeRect(outer_bounds.clone(), n, border_style));
                } else {
                    for edge in &edges {
                        let command = match edge {
                            layout::alignment::Edge::Top => {
                                let line_bounds = Rect::new(outer_bounds.x, outer_bounds.y, outer_bounds.width, n);
                                
                                DrawCommand::FillRect(line_bounds, border_style.clone())
                            }
                            layout::alignment::Edge::Right => {
                                let line_bounds = Rect::new(outer_bounds.max_x() - n as i64, outer_bounds.y, n, outer_bounds.height);
                                
                                DrawCommand::FillRect(line_bounds, border_style.clone())
                            }
                            layout::alignment::Edge::Bottom => {
                                let line_bounds = Rect::new(outer_bounds.x, outer_bounds.max_y() - n as i64, outer_bounds.width, n);
                                
                                DrawCommand::FillRect(line_bounds, border_style.clone())
                            }
                            layout::alignment::Edge::Left => {
                                let line_bounds = Rect::new(outer_bounds.x, outer_bounds.y, n, outer_bounds.height);
                                
                                DrawCommand::FillRect(line_bounds, border_style.clone())
                            }
                        };

                        commands.push(command);
                    }
                }

                commands
            }
            VerticalStack(alignment, spacing, nodes) => {
                let mut max_width = 0usize;
                
                let spacing_sizing = spacing * (nodes.len().saturating_sub(1));

                let mut last_bounds = Rect::zero();

                let mut greedy_count = 0;
                let mut static_height = spacing_sizing;

                for node in &nodes {
                    if let layout::sizing::Sizing::Static(n) = node.sizing.vertical {
                        static_height += n;
                    } else {
                        greedy_count += 1;
                    }
                }

                let mut greedy_space = bounds.height - static_height;
                let greedy_size = if greedy_count != 0 { greedy_space / greedy_count } else { 0 };

                let mut new_nodes = vec![];

                for node in &nodes {
                    let mut n = (*node).clone();
                    n.sizing.vertical = match n.sizing.vertical {
                        layout::sizing::Sizing::Static(sz) => layout::sizing::Sizing::Static(sz),
                        layout::sizing::Sizing::Greedy(tight) => {
                            greedy_space -= greedy_size;
                            let mut node_height = greedy_size;
                            if greedy_space < greedy_size {
                                node_height += greedy_space;
                                greedy_space = 0;
                            }

                            layout::sizing::Sizing::Static(node_height.max(tight))
                        }
                    };

                    new_nodes.push(n);
                }

                let nodes = new_nodes;

                let mut raw_bounds = vec![];
                for node in &nodes {
                    let size = node.sizing.fit_into(bounds);

                    let spacing_offset = if raw_bounds.is_empty() {
                        0
                    } else {
                        spacing as i64
                    };

                    let node_bounds = Rect::new(0, last_bounds.max_y() + spacing_offset, size.width, size.height);
                    last_bounds = node_bounds.clone();

                    if node_bounds.width > max_width {
                        max_width = node_bounds.width;
                    }

                    raw_bounds.push(node_bounds);
                }

                let final_bounds: Vec<_> = raw_bounds.into_iter().map(|mut bound| {
                    match &alignment {
                        layout::alignment::HorizontalAlignment::Left => { /* Already aligned to the left */}
                        layout::alignment::HorizontalAlignment::Center => {
                            let center = max_width / 2;
                            let start = center - bound.width/2;
                            bound.x = start as i64;
                        }
                        layout::alignment::HorizontalAlignment::Right => {
                            let right = max_width;
                            let start = right - bound.width;
                            bound.x = start as i64;
                        }
                    }

                    // move from 0 based bounds to the actual frame of the container
                    bound.x += bounds.x;
                    bound.y += bounds.y;

                    bound
                }).collect();

                

                nodes.into_iter().enumerate().flat_map(|(i, node)| {
                    let size = &final_bounds[i];

                    self.resolve_draw_commands(&node, size)
                }).collect::<Vec<_>>()
            }
            HorizontalStack(alignment, spacing, nodes) => {
                let mut max_height = 0usize;

                let spacing_sizing = spacing * (nodes.len().saturating_sub(1));

                let mut last_bounds = Rect::zero();

                let mut greedy_count = 0;
                let mut static_width = spacing_sizing;

                for node in &nodes {
                    if let layout::sizing::Sizing::Static(n) = node.sizing.horizontal {
                        static_width += n;
                    } else {
                        greedy_count += 1;
                    }
                }

                let mut greedy_space = bounds.width.saturating_sub(static_width);
                let greedy_size = if greedy_count != 0 { greedy_space / greedy_count } else { 0 };

                let mut new_nodes = vec![];

                for node in &nodes {
                    let mut n = node.clone();
                    n.sizing.horizontal = match n.sizing.horizontal {
                        layout::sizing::Sizing::Static(sz) => layout::sizing::Sizing::Static(sz),
                        layout::sizing::Sizing::Greedy(tight) => {
                            greedy_space -= greedy_size;
                            let mut node_width = greedy_size;
                            if greedy_space < greedy_size {
                                node_width += greedy_space;
                                greedy_space = 0;
                            }

                            layout::sizing::Sizing::Static(node_width.max(tight))
                        }
                    };

                    new_nodes.push(n);
                }

                let nodes = new_nodes;

                let mut raw_bounds = vec![];
                for node in &nodes {
                    let size = node.sizing.fit_into(bounds);

                    let spacing_offset = if raw_bounds.is_empty() {
                        0
                    } else {
                        spacing as i64
                    };

                    let node_bounds = Rect::new(last_bounds.max_x() + spacing_offset, 0, size.width, size.height);
                    last_bounds = node_bounds.clone();

                    if node_bounds.height > max_height {
                        max_height = node_bounds.height;
                    }

                    raw_bounds.push(node_bounds);
                }

                let final_bounds: Vec<_> = raw_bounds.into_iter().map(|mut bound| {
                    match &alignment {
                        layout::alignment::VerticalAlignment::Top => { /* Already aligned to the top */}
                        layout::alignment::VerticalAlignment::Center => {
                            let center = max_height / 2;
                            let start = center - bound.height/2;
                            bound.y = start as i64;
                        }
                        layout::alignment::VerticalAlignment::Bottom => {
                            let bottom = max_height;
                            let start = bottom - bound.height;
                            bound.y = start as i64;
                        }
                    }

                    // move from 0 based bounds to the actual frame of the container
                    bound.x += bounds.x;
                    bound.y += bounds.y;

                    bound
                }).collect();

                

                nodes.into_iter().enumerate().flat_map(|(i, node)| {
                    let size = &final_bounds[i];

                    self.resolve_draw_commands(&node, size)
                }).collect::<Vec<_>>()
            }
        }
    }
}