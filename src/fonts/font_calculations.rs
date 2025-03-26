use std::slice::Iter;

use crate::layout::geometry::{Rect, Size};

use super::{ascii::Ascii, Font, Glyph};

#[derive(Clone, PartialEq)]
pub struct ResolvedGlyph<'glyph> {
    offset: Size,
    glyph: &'glyph Glyph,
    size: Size
}

#[derive(Clone, PartialEq)]
pub enum ResolvedGlyphElement<'glyph> {
    Glyph(ResolvedGlyph<'glyph>),
    Space
}

#[derive(Clone)]
pub struct ResolvedLine<'source, 'glyph> {
    line_text: &'source str,
    pub glyphs: Vec<ResolvedGlyph<'glyph>>,
    bounds: Rect,
}

pub struct ResolvedText<'source, 'glyph> {
    lines: Vec<ResolvedLine<'source, 'glyph>>,
    size: Size,
}

impl<'glyph> ResolvedGlyph<'glyph> {
    fn new(offset: Size, glyph: &'glyph Glyph, size: Size) -> Self {
        ResolvedGlyph {
            offset,
            glyph,
            size
        }
    }

    pub fn glyph(&self) -> &'glyph Glyph {
        self.glyph
    }

    pub fn size(&self) -> &Size {
        &self.size
    }

    pub fn offset(&self) -> &Size {
        &self.offset
    }
}

impl<'source> ResolvedLine<'source, '_> {
    pub fn bounds(&self) -> &Rect {
        &self.bounds
    }

    pub fn line(&self) -> &'source str {
        self.line_text
    }
}

impl ResolvedText<'_, '_> {
    pub fn lines(&self) -> Iter<ResolvedLine> {
        self.lines.iter()
    }

    pub fn size(&self) -> &Size {
        &self.size
    }
}

impl Font {
    pub fn calculate_lines<'str, 'font>(
        &'font self,
        text: &'str str,
        bounds: &Rect,
    ) -> ResolvedText<'str, 'font> {
        let char_indices = text.char_indices();
        
        let mut context = LineResolution {
            fitting_lines: vec![],
            bounds,
            source_string: text,
            current_glyphs: vec![],
            font: self,
            character_height: 0,
            previous_starting_index: 0
        };

        let mut overflowing_y = false;

        for (index, c) in char_indices {
            if c == '\n' {
                context.commit_line(Some(index));
                continue;
            }
            
            let default_char = b'|';

            // FIXME: Calculating only first character. Unicode will not work
            let element_ascii = Ascii::try_from(c)
                .unwrap_or(Ascii(default_char));

            let element_glyph = self.get_glyph(&element_ascii);
            let element_size = self.size(element_ascii);
            
            context.commit_glyph(index, element_ascii, element_glyph, element_size.clone());

            let line_y = context.line_y();
            if line_y + element_size.height > bounds.height {
                overflowing_y = true;
                break;
            }
        }

        if !overflowing_y { context.commit_line(None); }

        let total_size = Size::new(context.content_width(), context.content_height());

        ResolvedText {
            lines: context.fitting_lines,
            size: total_size,
        }
    }
}

struct LineResolution<'source, 'font, 'bounds, 'glyph> {
    fitting_lines: Vec<ResolvedLine<'source, 'glyph>>,
    bounds: &'bounds Rect,
    source_string: &'source str,
    current_glyphs: Vec<ResolvedGlyphElement<'glyph>>,
    font: &'font Font,
    character_height: usize,
    previous_starting_index: usize
}

impl<'glyph> LineResolution<'_, '_, '_, 'glyph> {
    fn commit_line(&mut self, index_end: Option<usize>) {
        let subline;
        
        if let Some(index_end) = index_end {
            subline = self.source_string.get(self.previous_starting_index..index_end);
            self.previous_starting_index = index_end;
        } else {
            subline = self.source_string.get(self.previous_starting_index..);
            self.previous_starting_index = self.source_string.char_indices()
                .last().map(|x| x.0)
                .unwrap_or(0);
        };

        if let Some(subline) = subline {
            let line_x = 0;

            let glyphs = self.current_glyphs.iter().filter_map(|element| {
                match element {
                    ResolvedGlyphElement::Space => None,
                    ResolvedGlyphElement::Glyph(g) => Some(g.clone())
                }
            });

            let line_y = self.line_y();

            let resolved_line = ResolvedLine {
                line_text: subline,
                glyphs: glyphs.collect(),
                bounds: Rect::new(line_x, line_y as i64, self.current_line_width(), self.character_height), // FIXME: Line height will not work on non-mono
            };

            self.fitting_lines.push(resolved_line);
            self.current_glyphs = vec![];
        }
    }

    fn commit_glyph(&mut self, char_index: usize, element: Ascii, element_glyph: &'glyph Glyph, element_size: Size) {
        if element.is_space() {
            self.current_glyphs.push(ResolvedGlyphElement::Space);
            
            return
        }

        if self.character_height == 0 {
            self.character_height = element_size.height
        }
        
        let mut x_offset = self.next_glyph_x();

        // FIXME: Implement a better word wrapping/word breaking behavior.
        if (x_offset + element_size.width) > self.bounds.width {
            self.commit_line(Some(char_index));
            x_offset = 0;
        }

        let offset = Size::new(x_offset, 0);

        let resolved_glyph = ResolvedGlyph::new(
            offset,
            element_glyph,
            element_size
        );

        self.current_glyphs.push(ResolvedGlyphElement::Glyph(resolved_glyph));
    }

    pub fn next_glyph_x(&self) -> usize {
        let mut result = 0;

        let mut prev_was_glyph = false;

        for element in self.current_glyphs.iter() {
            match element {
                ResolvedGlyphElement::Space => {
                    result += self.font.space_width();

                    prev_was_glyph = false;
                },
                ResolvedGlyphElement::Glyph(glyph) => {
                    if prev_was_glyph {
                        result += self.font.character_spacing();
                    }
                    
                    result += glyph.size().width;

                    prev_was_glyph = true;
                }
            }
        }

        if prev_was_glyph {
            result += self.font.character_spacing();
        }

        result
    }

    pub fn current_line_width(&self) -> usize {
        let mut total: usize = 0;

        let mut prev_was_glyph = false;

        let mut last_before_spaces = 0;

        for element in self.current_glyphs.iter() {
            match element {
                ResolvedGlyphElement::Space => {
                    total += self.font.space_width();

                    prev_was_glyph = false;
                },
                ResolvedGlyphElement::Glyph(glyph) => {
                    if prev_was_glyph {
                        total += self.font.character_spacing();
                    }
                    
                    total += glyph.size().width;

                    prev_was_glyph = true;
                    last_before_spaces = total;
                }
            }
        }

        last_before_spaces
    }

    pub fn line_y(&self) -> usize {
        let max_y = self.fitting_lines
            .last()
            .map(|x| x.bounds.max_y() as usize)
            .unwrap_or(0);

        if max_y != 0 {
            max_y + self.font.line_spacing()
        } else {
            max_y
        }
    }

    fn content_width(&self) -> usize {
        let lines = &self.fitting_lines;
        let widths = lines.iter()
            .map(|line| line.bounds().width);

        widths.max().unwrap_or_default()
    }

    fn content_height(&self) -> usize {
        let all_lines_height = self.fitting_lines.iter().fold(0, |n, line|{
            n + line.bounds.height
        });
        let line_spacing_sum = self.font.line_spacing() * self.fitting_lines.len().saturating_sub(1);

        all_lines_height + line_spacing_sum
    }
}