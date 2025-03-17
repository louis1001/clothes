use std::fmt::Debug;

use crate::{fonts::{ascii::Ascii, Font}, layout::{self, alignment::{self, Edge}, sizing}, rendering::DrawCommand};

use super::{geometry::{Rect, Size}, node::Node, sized_node::{SizedNode, SizedItem}};

// Calculate size
pub struct SizeCalculator;

impl SizeCalculator {
    fn calculate_line_size(line: &str, bounds: &Rect, font: &Font) -> Size {
        let ascii_elements = line
            .chars()
            .filter_map(|x| Ascii::try_from(x).ok());
        
        let mut row_count = 1; // TODO: Are rows being calculated correctly? Too many off by one errors.
        let mut max_width: usize = 0;
        let mut current_width = 0;
        let mut character_height = 0;

        for element in ascii_elements {
            if element.0 == b' ' {
                // FIXME: Add character spacing before space?
                if current_width != 0 {
                    current_width += font.space_width();
                }
                continue;
            }

            let element_size = font.size(element);

            if character_height == 0 {
                // FIXME: Better height calculation
                character_height = element_size.height;
            }

            let character_spacing = if current_width != 0 { font.character_spacing() } else { 0 };
            let new_width = current_width + element_size.width + character_spacing;

            if new_width > bounds.width {
                current_width = element_size.width;
                row_count += 1;

                continue;
            }

            current_width = new_width;
            max_width = current_width.max(max_width);
        }

        let height = (row_count * character_height) + ((row_count - 1) * font.line_spacing());

        Size::new(max_width, height)
    }

    pub fn resolve_size<Content: Clone + Default + Debug, Ctx: Clone + std::fmt::Debug>(container_node: &Node<Content, Ctx>, bounds: &Rect, context: &mut Ctx) -> SizedNode<Content> {
        use Node::*;
        use sizing::Sizing::*;

        match container_node {
            Text(t) => {
                let font = Font::singleton();
                let lines = t.lines();

                let mut width = 0usize;
                let mut height = 0usize;
                for line in lines {
                    let sz = Self::calculate_line_size(line, bounds, font);
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
                let resolved = Self::resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_height = content_size.vertical.min_content_size();

                let sizing = sizing::ItemSizing::new(content_size.horizontal, Greedy(min_height));

                SizedNode::new(SizedItem::VCenter(resolved), sizing)
            }
            VBottomAlign(node) => {
                let resolved = Self::resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_height = content_size.vertical.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: content_size.horizontal, vertical: Greedy(min_height) };

                SizedNode::new(SizedItem::VBottomAlign(resolved), sizing)
            }
            HCenter(node) => {
                let resolved = Self::resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_width = content_size.horizontal.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: Greedy(min_width), vertical: content_size.vertical };

                SizedNode::new(SizedItem::HCenter(resolved), sizing)
            }
            HRightAlign(node) => {
                let resolved = Self::resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_width = content_size.horizontal.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: Greedy(min_width), vertical: content_size.vertical };

                SizedNode::new(SizedItem::HRightAlign(resolved), sizing)
            }
            VTopAlign(node) => {
                let resolved = Self::resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_height = content_size.vertical.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: content_size.horizontal, vertical: Greedy(min_height) };

                SizedNode::new(SizedItem::VTopAlign(resolved), sizing)
            }
            HLeftAlign(node) => {
                let resolved = Self::resolve_size(node, bounds, context);
                let content_size = resolved.sizing.clone();

                let min_width = content_size.horizontal.min_content_size();

                let sizing = sizing::ItemSizing { horizontal: Greedy(min_width), vertical: content_size.vertical };

                SizedNode::new(SizedItem::HLeftAlign(resolved), sizing)
            }
            Width(size, node) => {
                let mut bounds = bounds.clone();
                bounds.width = *size;

                let resolved_content = Self::resolve_size(node, &bounds, context);
                let mut frame = resolved_content.sizing.clone();
                frame.horizontal = Static(*size);

                SizedNode::new(SizedItem::Width(*size, resolved_content), frame)
            }
            Height(size, node) => {
                let mut bounds = bounds.clone();
                bounds.height = *size;

                let resolved_content = Self::resolve_size(node, &bounds, context);
                let mut frame = resolved_content.sizing.clone();
                frame.vertical = Static(*size);

                SizedNode::new(SizedItem::Height(*size, resolved_content), frame)
            }
            TopPadding(n, node) | BottomPadding(n, node) => {
                let resolved = Self::resolve_size(node, bounds, context);
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

                    let resolved_content = Self::resolve_size(node, &bounds, context);
                    let mut frame = resolved_content.sizing.clone();

                    frame.vertical.clamped_add(*n);

                    SizedNode::new(make_node(*n, resolved_content), frame)
                } else {
                    SizedNode::new(make_node(*n, resolved), frame)
                }
            }
            LeftPadding(n, node) | RightPadding(n, node) => {
                let resolved = Self::resolve_size(node, bounds, context);
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

                    let resolved_content = Self::resolve_size(node, &bounds, context);
                    frame = resolved_content.sizing.clone();
                    frame.horizontal.clamped_add(*n);

                    let node = make_node(*n, resolved_content);

                    SizedNode::new(node, frame)
                } else {
                    SizedNode::new(make_node(*n, resolved), frame)
                }
            }
            Background(c, node) => {
                let resolved_content = Self::resolve_size(node, bounds, context);
                let frame = resolved_content.sizing.clone();

                SizedNode::new(SizedItem::Background(c.clone(), resolved_content), frame)
            }
            TopBorder(n, c, node) => {
                let outer_bounds = bounds;
                let mut resolved_content = Self::resolve_size(node, outer_bounds, context);
                let mut frame = resolved_content.sizing.clone();

                let added_height = *n;
                frame.vertical.clamped_add(*n);

                if frame.vertical.min_content_size() > outer_bounds.height {
                    // recalculate with less space
                    let mut bounds = outer_bounds.clone();
                    bounds.height = bounds.height.saturating_sub(added_height);

                    resolved_content = Self::resolve_size(node, &bounds, context);
                    frame = resolved_content.sizing.clone();

                    frame.vertical.clamped_add(added_height);
                }

                SizedNode::new(SizedItem::Border(*n, c.clone(), dictionary!(Edge::Top), resolved_content), frame)
            }
            BottomBorder(n, c, node) => {
                let outer_bounds = bounds;
                let mut resolved_content = Self::resolve_size(node, outer_bounds, context);
                let mut frame = resolved_content.sizing.clone();

                let added_height = *n;
                frame.vertical.clamped_add(*n);

                if frame.vertical.min_content_size() > outer_bounds.height {
                    // recalculate with less space
                    let mut bounds = outer_bounds.clone();
                    bounds.height = bounds.height.saturating_sub(added_height);

                    resolved_content = Self::resolve_size(node, &bounds, context);
                    frame = resolved_content.sizing.clone();

                    frame.vertical.clamped_add(added_height);
                }

                SizedNode::new(SizedItem::Border(*n, c.clone(), dictionary!(Edge::Bottom), resolved_content), frame)
            }
            LeftBorder(n, c, node) => {
                let outer_bounds = bounds;
                let mut resolved_content = Self::resolve_size(node, outer_bounds, context);
                let mut frame = resolved_content.sizing.clone();

                let added_width = *n;
                frame.horizontal.clamped_add(*n);
                
                if frame.horizontal.min_content_size() > outer_bounds.width {
                    // recalculate with less space
                    let mut bounds = outer_bounds.clone();
                    bounds.width = bounds.width.saturating_sub(added_width);

                    resolved_content = Self::resolve_size(node, &bounds, context);
                    frame = resolved_content.sizing.clone();

                    frame.horizontal.clamped_add(added_width);
                }

                SizedNode::new(SizedItem::Border(*n, c.clone(), dictionary!(Edge::Left), resolved_content), frame)
            }
            RightBorder(n, c, node) => {
                let outer_bounds = bounds;
                let mut resolved_content = Self::resolve_size(node, outer_bounds, context);
                let mut frame = resolved_content.sizing.clone();

                let added_width = *n;
                frame.horizontal.clamped_add(*n);

                if frame.horizontal.min_content_size() > outer_bounds.width {
                    // recalculate with less space
                    let mut bounds = outer_bounds.clone();
                    bounds.width = bounds.width.saturating_sub(added_width);

                    resolved_content = Self::resolve_size(node, &bounds, context);
                    frame = resolved_content.sizing.clone();

                    frame.horizontal.clamped_add(added_width);
                }

                SizedNode::new(SizedItem::Border(*n, c.clone(), dictionary!(Edge::Right), resolved_content), frame)
            }

            VerticalStack(alignment, spacing,  nodes) => {
                let spacing_sizing = spacing * nodes.len().saturating_sub(1);
                let mut result = sizing::ItemSizing { horizontal: Static(0), vertical: Static(spacing_sizing) };
                let mut bounds = bounds.clone();
                bounds.height = bounds.height.saturating_sub(spacing_sizing);
                let mut resolved_children: Vec<SizedNode<_>> = vec![];

                for node in nodes {
                    let resolved_node = Self::resolve_size(node, &bounds, context);
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
                    let resolved_node = Self::resolve_size(node, &bounds, context);
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

                Self::resolve_size(&node, bounds, context)
            }
        }
    }
}

// Resolve size
pub struct SizeResolver;

impl SizeResolver {
    fn split_wrapping<'a>(line: &'a str, bounds: &Rect, font: &Font) -> Vec<&'a str> {
        let mut wrapping_lines = vec![];

        let mut current_width = 0;

        let char_indices = line.char_indices();

        let Some((mut previous_starting_index, _)) = char_indices.clone().next() else {
            return vec![];
        };

        for (index, c) in char_indices {
            // FIXME: Calculating only first character. Unicode will not work
            let Ok(element) = Ascii::try_from(c) else {
                // TODO: Default unknown character
                continue;
            };
            
            if element.0 == b' ' {
                // FIXME: Add character spacing before space?
                if current_width != 0 {
                    current_width += font.space_width();
                }
                continue;
            }

            let element_size = font.size(element);

            let character_spacing = if current_width != 0 { font.character_spacing() } else { 0 };
            let new_width = current_width + element_size.width + character_spacing;

            if new_width > bounds.width {
                let subline = line.get(previous_starting_index..index);
                previous_starting_index = index;
                current_width = element_size.width;
                
                if let Some(subline) = subline {
                    wrapping_lines.push(subline);
                }

                continue;
            }

            current_width = new_width;
        }

        let last_line = line.get(previous_starting_index..);

        if let Some(last_line) = last_line {
            if last_line.len() > 0 {
                wrapping_lines.push(last_line);
            }
        }

        wrapping_lines
    }

    fn calculate_line_size(line: &str, font: &Font) -> Size {
        let ascii_elements = line
            .chars()
            .filter_map(|x| Ascii::try_from(x).ok());

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

            current_width = current_width + font.character_spacing() + element_size.width;
            max_width = current_width.max(max_width);
        }

        Size::new(max_width, character_height)
    }

    pub fn resolve_draw_commands<Content: Clone + Default>(sized_node: &SizedNode<Content>, bounds: &Rect) -> Vec<DrawCommand<Content>> {
        use SizedItem::*;
        let layout = sized_node.clone();

        match *layout.node {
            Text(text) => {
                // TODO: Handle current content (foreground, background style)
                let font = Font::singleton();
                let lines = text.lines()
                    .flat_map(|line| Self::split_wrapping(line, bounds, font));

                let mut commands = vec![];
                let mut line_y = bounds.y;

                for line in lines {
                    let size = Self::calculate_line_size(line, font);

                    let line_bounds = Rect::new(
                        bounds.x, // FIXME: Left align
                        line_y,
                        size.width,
                        size.height
                    );

                    line_y += line_bounds.height as i64 + font.line_spacing() as i64;

                    commands.push(DrawCommand::TextLine(line_bounds, line.to_string(), Content::default()));
                }

                // TODO: Convert text into text lines (split by new line, handle when a line wraps...)
                commands
            }
            Width(_, node) | Height(_, node) => {
                let frame = node.sizing.fit_into(bounds);

                Self::resolve_draw_commands(&node, &frame)
            }
            VCenter(n) => {
                let mut content_rect = n.sizing.fit_into(bounds);
                let center_pos = bounds.y as usize + bounds.height / 2;
                let center_start = center_pos - content_rect.height / 2;
                content_rect.y = center_start as i64;

                let content_bounds = n.sizing.fit_into(&content_rect);

                Self::resolve_draw_commands(&n, &content_bounds)
            }
            HCenter(n) => {
                let mut content_rect = n.sizing.fit_into(bounds);
                let center_pos = bounds.x as usize + bounds.width / 2;
                let center_start = center_pos - content_rect.width / 2;
                content_rect.x = center_start as i64;

                let content_bounds = n.sizing.fit_into(&content_rect);

                Self::resolve_draw_commands(&n, &content_bounds)
            }
            VBottomAlign(n) => {
                let mut content_rect = n.sizing.fit_into(bounds);
                let bottom_most = bounds.y as usize + bounds.height;
                let top_start = bottom_most - content_rect.height;
                content_rect.y = top_start as i64;

                Self::resolve_draw_commands(&n, &content_rect)
            }
            HRightAlign(n) => {
                let mut content_rect = n.sizing.fit_into(bounds);
                let right_most = bounds.x as usize + bounds.width;
                let left_start = right_most - content_rect.width;
                content_rect.x = left_start as i64;

                let content_bounds = n.sizing.fit_into(&content_rect);

                Self::resolve_draw_commands(&n, &content_bounds)
            }
            VTopAlign(n) | HLeftAlign(n) => {
                let content_rect = n.sizing.fit_into(bounds);

                Self::resolve_draw_commands(&n, &content_rect)
            }
            TopPadding(n, node) => {
                let mut bounds = bounds.clone();
                bounds.height = bounds.height.saturating_sub(n);
                let mut frame = node.sizing.fit_into(&bounds);
                frame.x = bounds.x;
                frame.y = bounds.y + n as i64;

                Self::resolve_draw_commands(&node, &frame)
            }
            BottomPadding(n, node) => {
                let mut bounds = bounds.clone();
                bounds.height = bounds.height.saturating_sub(n);

                let mut frame = node.sizing.fit_into(&bounds);
                frame.x = bounds.x;
                frame.y = bounds.y;

                Self::resolve_draw_commands(&node, &frame)
            }
            RightPadding(n, node) => {
                let mut frame = node.sizing.fit_into(bounds);
                frame.x = bounds.x;
                frame.y = bounds.y;

                let free_width = bounds.width.saturating_sub(n);
                let adjustment = frame.width.saturating_sub(free_width);

                frame.width = frame.width.saturating_sub(adjustment);

                Self::resolve_draw_commands(&node, &frame)
            }
            LeftPadding(n, node) => {
                let mut bounds = bounds.clone();
                bounds.width = bounds.width.saturating_sub(n);
                let mut frame = node.sizing.fit_into(&bounds);
                frame.x = bounds.x + n as i64;
                frame.y = bounds.y;

                Self::resolve_draw_commands(&node, &frame)
            }
            Background(background_style, node) => {
                let mut frame = node.sizing.fit_into(bounds);
                frame.x = bounds.x;
                frame.y = bounds.y;

                let mut commands = vec![DrawCommand::FillRect(bounds.clone(), background_style)];

                let content_commands = Self::resolve_draw_commands(&node, &frame);

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

                let mut commands = Self::resolve_draw_commands(&node, &frame);

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

                    Self::resolve_draw_commands(&node, size)
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

                    Self::resolve_draw_commands(&node, size)
                }).collect::<Vec<_>>()
            }
        }
    }
}