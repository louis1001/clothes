pub mod ascii;

use std::collections::HashMap;

use crate::layout::geometry::Size;
use ascii::Ascii;
use once_cell::sync::Lazy;

/// A grid of points (bool) that determine how a glyph should be drawn.
/// `true` is a pixel to fill in, `false` represents an empty space in the grid.
pub struct Glyph(Vec<bool>);

pub struct Font {
    glyphs: HashMap<Ascii, Glyph>,
    unknown_glyph: Glyph,
    size: Size,
    space_width: usize,
    line_spacing: usize,
    character_spacing: usize
}

static THREE_BY_THREE_MONO: Lazy<Font> = Lazy::new(|| {
    Font {
        glyphs: three_by_three_glyphs(),
        size: Size::new(3, 3),
        unknown_glyph: three_by_three_unknown_glyph(),
        space_width: 2,
        line_spacing: 1,
        character_spacing: 1
    }
});

impl Font {
    pub fn singleton() -> &'static Self {
        // TODO: Hardcoded 3x3 font
        &THREE_BY_THREE_MONO
    }

    pub fn get_glyph(&self, c: &Ascii) -> &Glyph {
        self.glyphs.get(c).unwrap_or(&self.unknown_glyph)
    }

    pub fn space_width(&self) -> usize {
        self.space_width
    }

    pub fn line_spacing(&self) -> usize {
        self.line_spacing
    }

    pub fn character_spacing(&self) -> usize {
        self.character_spacing
    }

    pub fn default_glyph(&self) -> &Glyph {
        &self.unknown_glyph
    }

    pub fn size(&self, _c: Ascii) -> &Size {
        // TODO: Handle non-monospaced fonts (by using the actual character)
        &self.size
    }
}

// TODO: use this. For now, will only use simple 3x3 font
pub enum FontType {
    Bitmap
}

fn three_by_three_glyphs() -> HashMap<Ascii, Glyph> {
    let mut glyphs: HashMap<Ascii, Glyph> = Default::default();
    let mut insert = |chars: Vec<char>, points: Vec<bool>| {
        for c in chars {
            let Ok(ascii) = c.try_into() else { continue };
            glyphs.insert(ascii, Glyph(points.clone()));
        }
    };

    // FIXME: Ugh
    insert(vec!['1'],vec![true, true, false, false, true, false, true, true, true]);
    insert(vec!['3'],vec![true, true, true, false, true, true, true, true, true]);
    insert(vec!['4'],vec![true, false, true, true, true, true, false, false, true]);
    insert(vec!['6'],vec![true, false, false, true, true, true, true, true, true]);
    insert(vec!['7'],vec![true, true, true, false, false, true, false, false, true]);
    insert(vec!['8'],vec![true, true, false, true, true, true, false, true, true]);
    insert(vec!['9'],vec![true, true, true, true, true, true, false, false, true]);

    insert(vec!['a', 'A'],vec![false, true, false, true, true, true, true, false, true]);
    insert(vec!['b', 'B'],vec![true, true, false, true, true, true, true, true, true]);
    insert(vec!['c', 'C'],vec![true, true, true, true, false, false, true, true, true]);
    insert(vec!['d', 'D'],vec![true, true, false, true, false, true, true, true, false]);
    insert(vec!['e', 'E'],vec![true, true, true, true, true, false, true, true, true]);
    insert(vec!['f', 'F'],vec![true, true, true, true, true, false, true, false, false]);
    insert(vec!['g', 'G'],vec![true, true, false, true, false, true, true, true, true]);
    insert(vec!['h', 'H'],vec![true, false, true, true, true, true, true, false, true]);
    insert(vec!['i', 'I'],vec![true, true, true, false, true, false, true, true, true]);
    insert(vec!['j', 'J'],vec![false, false, true, true, false, true, true, true, true]);
    insert(vec!['k', 'K'],vec![true, false, true, true, true, false, true, false, true]);
    insert(vec!['l', 'L'],vec![true, false, false, true, false, false, true, true, true]);
    insert(vec!['m', 'M'],vec![true, true, true, true, true, true, true, false, true]);
    insert(vec!['n', 'N'],vec![true, true, false, true, false, true, true, false, true]);
    insert(vec!['o', 'O', '0'],vec![true, true, true, true, false, true, true, true, true]);
    insert(vec!['p', 'P'],vec![true, true, true, true, true, true, true, true, false]);
    insert(vec!['q', 'Q'],vec![true, true, true, true, true, true, false, true, true]);
    insert(vec!['r', 'R'],vec![true, true, true, true, false, false, true, false, false]);
    insert(vec!['s', 'S', '5'],vec![false, true, true, false, true, false, true, true, false]);
    insert(vec!['t', 'T'],vec![true, true, true, false, true, false, false, true, false]);
    insert(vec!['u', 'U'],vec![true, false, true, true, false, true, true, true, true]);
    insert(vec!['v', 'V'],vec![true, false, true, true, false, true, false, true, false]);
    insert(vec!['w', 'W'],vec![true, false, true, true, true, true, true, true, true]);
    insert(vec!['x', 'X'],vec![true, false, true, false, true, false, true, false, true]);
    insert(vec!['y', 'Y'],vec![true, false, true, false, true, false, false, true, false]);
    insert(vec!['z', 'Z', '2'],vec![true, true, false, false, true, false, false, true, true]);
    
    insert(vec!['?'],vec![true, true, true, false, false, true, false, true, false]);
    insert(vec!['!'],vec![true, true, true, false, false, false, false, true, false]);
    insert(vec![':'],vec![false, true, false, false, false, false, false, true, false]);
    insert(vec![';'],vec![false, true, false, false, false, false, false, true, true]);
    insert(vec!['/'],vec![false, false, true, false, true, false, true, false, false]);
    insert(vec!['\\'],vec![true, false, true, false, true, false, false, false, true]);
    insert(vec!['.'], vec![false, false, false, false, false, false, true, false, false]);
    insert(vec![','], vec![false, false, false, false, true, false, true, false, false]);

    return glyphs
}

fn three_by_three_unknown_glyph() -> Glyph {
    Glyph(vec![false, true, false, true, false, true, false, true, false])
}

impl Glyph {
    pub fn map<V, F: Fn(bool) -> V>(&self, transform: F) -> Vec<V> {
        self.0.iter().map(| b| transform(*b)).collect()
    }
}