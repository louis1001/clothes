pub mod ascii;
pub mod font_calculations;

use std::collections::HashMap;

use crate::layout::geometry::Size;
use ascii::Ascii;
use once_cell::sync::Lazy;

/// A grid of points (bool) that determine how a glyph should be drawn.
/// `true` is a pixel to fill in, `false` represents an empty space in the grid.
#[derive(PartialEq, Debug)]
pub struct Glyph(Vec<bool>);

#[derive(Debug)]
pub struct Font {
    pub name: String,
    glyphs: HashMap<Ascii, Glyph>,
    unknown_glyph: Glyph,
    size: Size,
    space_width: usize,
    line_spacing: usize,
    character_spacing: usize
}

static THREE_BY_THREE_MONO: Lazy<Font> = Lazy::new(|| {
    let square_size = 3;
    let character_spacing = 1;
    let space_width = character_spacing * 2 + square_size;

    Font {
        name: "3x3Mono".to_string(),
        glyphs: three_by_three_glyphs(),
        size: Size::new(square_size, square_size),
        unknown_glyph: three_by_three_unknown_glyph(),
        space_width,
        line_spacing: 1,
        character_spacing
    }
});

static FOUR_BY_FOUR_MONO: Lazy<Font> = Lazy::new(|| {
    let square_size = 4;
    let character_spacing = 1;
    let space_width = character_spacing * 2 + square_size;

    Font {
        name: "4x4Mono".to_string(),
        glyphs: four_by_four_glyphs(),
        size: Size::new(square_size, square_size),
        unknown_glyph: four_by_four_unknown_glyph(),
        space_width,
        line_spacing: 1,
        character_spacing
    }
});

static FOUR_BY_FIVE_MONO: Lazy<Font> = Lazy::new(|| {
    let character_width = 4;
    let character_height = 5;
    let character_spacing = 1;
    let space_width = character_spacing * 2 + character_width;

    Font {
        name: "4x5Mono".to_string(),
        glyphs: four_by_five_glyphs(),
        size: Size::new(character_width, character_height),
        unknown_glyph: four_by_five_unknown_glyph(),
        space_width,
        line_spacing: 1,
        character_spacing
    }
});

impl Font {
    pub fn three_by_three() -> &'static Self {
        // TODO: Hardcoded 3x3 font
        &THREE_BY_THREE_MONO
    }

    pub fn four_by_four() -> &'static Self {
        // TODO: Hardcoded 4x4 font
        &FOUR_BY_FOUR_MONO
    }

    pub fn four_by_five() -> &'static Self {
        // TODO: Hardcoded 4x5 font
        &FOUR_BY_FIVE_MONO
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
    insert(vec!['\''], vec![false, true, false, false, true, false, false, false, false]);
    insert(vec!['"'], vec![true, false, true, true, false, true, false, false, false]);
    insert(vec!['_'], vec![false, false, false, false, false, false, true, true, true]);
    insert(vec!['('], vec![false, true, false, true, false, false, false, true, false]);
    insert(vec![')'], vec![false, true, false, false, false, true, false, true, false]);
    insert(vec!['='], vec![true, true, true, false, false, false, true, true, true]);
    
    insert(vec!['+'], vec![false, true, false, true, true, true, false, true, false]);
    insert(vec!['-'], vec![false, false, false, true, true, true, false, false, false]);

    glyphs
}

fn three_by_three_unknown_glyph() -> Glyph {
    Glyph(vec![false, true, false, true, false, true, false, true, false])
}

fn four_by_four_glyphs() -> HashMap<Ascii, Glyph> {
    let mut glyphs: HashMap<Ascii, Glyph> = Default::default();
    let mut insert = |chars: Vec<char>, points: Vec<bool>| {
        for c in chars {
            let Ok(ascii) = c.try_into() else { continue };
            glyphs.insert(ascii, Glyph(points.clone()));
        }
    };

    insert(vec!['1'],vec![]);
    insert(vec!['3'],vec![]);
    insert(vec!['4'],vec![]);
    insert(vec!['6'],vec![]);
    insert(vec!['7'],vec![]);
    insert(vec!['8'],vec![]);
    insert(vec!['9'],vec![]);

    insert(vec!['a', 'A'],vec![true, true, true, false, true, false, false, true, true, true, true, true, true, false, false, true]);
    insert(vec!['b', 'B'],vec![true, true, true, false, true, true, true, true, true, false, false, true, true, true, true, true]);
    insert(vec!['c', 'C'],vec![false, true, true, true, true, true, false, false, true, true, false, false, false, true, true, true]);
    insert(vec!['d', 'D'],vec![true, true, true, false, true, false, false, true, true, false, false, true, true, true, true, false]);
    insert(vec!['e', 'E'],vec![true, true, true, true, true, true, true, false, true, false, false, false, true, true, true, true]);
    insert(vec!['f', 'F'],vec![true, true, true, true, true, false, false, false, true, true, true, false, true, false, false, false]);
    insert(vec!['g', 'G'],vec![true, true, true, true, true, false, false, false, true, false, true, true, true, true, true, true]);
    insert(vec!['h', 'H'],vec![true, false, false, true, true, true, true, true, true, false, false, true, true, false, false, true]);
    insert(vec!['i', 'I'],vec![true, true, true, true, false, true, false, false, false, true, false, false, true, true, true, true]);
    insert(vec!['j', 'J'],vec![true, true, true, true, false, false, true, false, false, false, true, false, true, true, false, false]);
    insert(vec!['k', 'K'],vec![true, false, false, true, true, false, true, true, true, true, true, false, true, false, false, true]);
    insert(vec!['l', 'L'],vec![true, false, false, false, true, false, false, false, true, false, false, false, true, true, true, true]);
    insert(vec!['m', 'M'],vec![true, false, true, true, true, true, true, true, true, true, false, true, true, false, false, true]);
    insert(vec!['n', 'N'],vec![true, false, false, true, true, true, false, true, true, false, true, true, true, false, false, true]);
    insert(vec!['o', 'O', '0'],vec![false, true, true, false, true, false, false, true, true, false, false, true, false, true, true, false]);
    insert(vec!['p', 'P'],vec![true, true, true, false, true, false, false, true, true, true, true, false, true, false, false, false]);
    insert(vec!['q', 'Q'],vec![false, true, true, false, true, false, false, true, true, false, true, true, false, true, true, true]);
    insert(vec!['r', 'R'],vec![true, true, true, false, true, false, false, true, true, true, true, false, true, false, false, true]);
    insert(vec!['s', 'S', '5'],vec![true, true, true, true, true, true, false, false, false, true, true, true, true, true, true, true]);
    insert(vec!['t', 'T'],vec![true, true, true, true, false, true, false, false, false, true, false, false, false, true, false, false]);
    insert(vec!['u', 'U'],vec![true, false, false, true, true, false, false, true, true, false, false, true, false, true, true, false]);
    insert(vec!['v', 'V'],vec![true, false, false, true, true, false, false, true, true, false, true, false, false, true, false, false]);
    insert(vec!['w', 'W'],vec![true, false, false, true, true, false, true, true, true, true, true, true, true, true, false, true]);
    insert(vec!['x', 'X'],vec![true, false, false, true, false, true, true, false, true, false, false, true, true, false, false, true]);
    insert(vec!['y', 'Y'],vec![true, false, false, true, true, false, false, true, false, true, true, false, false, false, true, false]);
    insert(vec!['z', 'Z', '2'],vec![true, true, true, true, false, false, true, true, true, true, true, false, true, true, true, true]);
    
    // insert(vec!['?'],vec![]);
    // insert(vec!['!'],vec![]);
    // insert(vec![':'],vec![]);
    // insert(vec![';'],vec![]);
    // insert(vec!['/'],vec![]);
    // insert(vec!['\\'],vec![]);
    // insert(vec!['.'], vec![]);
    // insert(vec![','], vec![]);
    // insert(vec!['\''], vec![]);
    // insert(vec!['"'], vec![]);
    // insert(vec!['_'], vec![]);
    // insert(vec!['('], vec![]);
    // insert(vec![')'], vec![]);
    // insert(vec!['='], vec![]);
    
    // insert(vec!['+'], vec![]);
    // insert(vec!['-'], vec![]);

    glyphs
}

fn four_by_four_unknown_glyph() -> Glyph {
    Glyph(vec![true, false, true, false, false, true, false, true, true, false, true, false, false, true, false, true,])
}

fn four_by_five_glyphs() -> HashMap<Ascii, Glyph> {
    let mut glyphs: HashMap<Ascii, Glyph> = Default::default();

    let map_4x4_to_4x5 = |glyph: &Glyph| {
        let mut new_points = vec![false, false, false, false];
        new_points.extend_from_slice(&glyph.0);

        let new_glyph = Glyph(new_points);

        new_glyph
    };

    // The 4x5 font uses 4x4 glyphs as the lowercase (and fallback for any non existent)
    four_by_four_glyphs()
        .convert(&mut glyphs, map_4x4_to_4x5);

    let mut insert = |chars: Vec<char>, points: Vec<bool>| {
        for c in chars {
            let Ok(ascii) = c.try_into() else { continue };
            glyphs.insert(ascii, Glyph(points.clone()));
        }
    };

    insert(vec!['A'],vec![true, true, true, false, true, false, false, true, true, true, true, true, true, false, false, true, true, false, false, true]);
    insert(vec!['B'],vec![true, true, true, false, true, false, false, true, true, true, true, false, true, false, false, true, true, true, true, true]);
    insert(vec!['C'],vec![false, true, true, false, true, false, true, false, true, false, false, false, true, false, false, true, false, true, true, false]);
    insert(vec!['D'],vec![true, true, true, false, true, false, false, true, true, false, false, true, true, false, false, true, true, true, true, false]);
    insert(vec!['E'],vec![true, true, true, true, true, false, false, false, true, true, true, false, true, false, false, false, true, true, true, true]);
    insert(vec!['F'],vec![true, true, true, true, true, false, false, false, true, true, true, false, true, false, false, false, true, false, false, false]);
    insert(vec!['G'],vec![true, true, true, true, true, false, false, false, true, false, true, true, true, false, false, true, true, true, true, false]);
    insert(vec!['H'],vec![true, false, false, true, true, false, false, true, true, true, true, true, true, false, false, true, true, false, false, true]);
    insert(vec!['I'],vec![true, true, true, true, false, true, false, false, false, true, false, false, false, true, false, false, true, true, true, true]);
    insert(vec!['J'],vec![true, true, true, true, false, false, true, false, false, false, true, false, true, false, true, false, false, true, true, false]);
    insert(vec!['K'],vec![true, false, false, true, true, false, true, false, true, true, false, false, true, false, true, false, true, false, false, true]);
    insert(vec!['L'],vec![true, false, false, false, true, false, false, false, true, false, false, false, true, false, false, false, true, true, true, true]);
    insert(vec!['M'],vec![true, false, true, true, true, true, true, true, true, true, false, true, true, false, false, true, true, false, false, true]);
    insert(vec!['N'],vec![true, false, false, true, true, true, false, true, true, true, true, true, true, false, true, true, true, false, false, true]);
    insert(vec!['O'],vec![false, true, true, false, true, false, false, true, true, false, false, true, true, false, false, true, false, true, true, false]);
    insert(vec!['P'],vec![true, true, true, false, true, false, false, true, true, true, true, false, true, false, false, false, true, false, false, false]);
    insert(vec!['Q'],vec![false, true, true, false, true, false, false, true, true, false, false, true, true, false, true, true, false, true, true, true]);
    insert(vec!['R'],vec![true, true, true, false, true, false, false, true, true, true, true, false, true, false, false, true, true, false, false, true]);
    insert(vec!['S'],vec![false, true, true, true, true, false, false, false, false, true, true, false, false, false, false, true, true, true, true, false]);
    insert(vec!['T'],vec![true, true, true, true, false, true, false, false, false, true, false, false, false, true, false, false, false, true, false, false]);
    insert(vec!['U'],vec![true, false, false, true, true, false, false, true, true, false, false, true, true, false, false, true, false, true, true, false]);
    insert(vec!['V'],vec![true, false, false, true, true, false, false, true, true, false, false, true, true, false, true, false, false, true, false, false]);
    insert(vec!['W'],vec![true, false, false, true, true, false, false, true, true, false, true, true, true, true, true, true, true, true, false, true]);
    insert(vec!['X'],vec![true, false, false, true, false, true, true, false, false, true, true, false, true, false, false, true, true, false, false, true]);
    insert(vec!['Y'],vec![true, false, false, true, true, false, false, true, false, true, true, false, false, false, true, false, false, false, true, false]);
    insert(vec!['Z'],vec![true, true, true, true, false, false, false, true, false, true, true, false, true, false, false, false, true, true, true, true]);

    insert(vec!['.'], vec![false, false, false, false, false, false, false, false, false, false, false, false, false, true, true, false, false, true, true, false]);
    insert(vec![','], vec![false, false, false, false, false, false, false, false, false, false, false, false, false, false, true, false, true, true, false, false]);
    insert(vec!['!'], vec![false, false, true, false, false, false, true, false, false, false, true, false, false, false, false, false, false, false, true, false]);
    insert(vec!['?'], vec![false, true, true, false, true, false, false, true, false, false, true, false, false, false, false, false, false, false, true, false]);


    glyphs
}

fn four_by_five_unknown_glyph() -> Glyph {
    Glyph(vec![true, false, true, false, false, true, false, true, true, false, true, false, false, true, false, true, true, false, true, false])
}

impl Glyph {
    pub fn scale(&self, original_size: &Size, scale: usize) -> Glyph {
        let mut new_points = vec![false; original_size.width * scale * original_size.height * scale];

        let new_size = original_size.clone().scaled(scale);

        for j in 0..new_size.height {
            for i in 0..new_size.width {
                let new_index = j * original_size.width + i;

                let original_i = i / scale;
                let original_j = i / scale;

                let original_index = original_j * original_size.width + original_i;

                let Some(item) = self.0.get(original_index) else { continue };

                new_points[new_index] = *item;
            }
        }

        Glyph(new_points)
    }

    pub fn map<V, F: Fn(bool) -> V>(&self, transform: F) -> Vec<V> {
        self.0.iter().map(| b| transform(*b)).collect()
    }
}

trait HashMapConvert {
    type K: Eq + std::hash::Hash + Copy;
    type V;
    fn convert<T, M: Fn(&Self::V) -> T>(self, other: &mut HashMap<Self::K, T>, mapper: M);
}

impl<K: Eq + std::hash::Hash + Copy, V> HashMapConvert for HashMap<K, V> {
    type K = K;
    type V = V;

    fn convert<T, M: Fn(&Self::V) -> T>(self, other: &mut HashMap<K, T>, mapper: M) {
        for key in self.keys() {
            let Some(value) = self.get(key) else { continue };
            let value = mapper(value);

            other.insert(*key, value);
        }
    }
}