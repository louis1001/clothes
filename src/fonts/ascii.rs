use std::{char::TryFromCharError, error::Error, fmt::Display};

#[derive(Clone, Copy, Hash, Eq, PartialEq)]
pub struct Ascii(pub u8);

impl Ascii {
    pub fn char(self) -> char {
        self.into()
    }

    pub fn u8(self) -> u8 {
        self.0
    }
}

impl From<u8> for Ascii {
    fn from(value: u8) -> Self {
        Ascii(value)
    }
}

impl From<Ascii>  for char {
    fn from(val: Ascii) -> Self {
        val.0 as char
    }
}

impl TryFrom<char> for Ascii {
    type Error = AsciiError;

    fn try_from(value: char) -> Result<Self, AsciiError> {
        if value.is_ascii() {
            value
                .try_into()
                .map(Self)
                .map_err(AsciiError::ConversionError)
        } else {
            Err(AsciiError::ValueNotAscii(value))
        }
    }
}

#[derive(Debug)]
pub enum AsciiError {
    ConversionError(TryFromCharError),
    ValueNotAscii(char)
}

impl Error for AsciiError {}

impl Display for AsciiError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{:?}", self))
    }
}

// Utilities

impl Ascii {
    pub fn is_printable(&self) -> bool {
        self.0 >= b' ' && self.0 <= b'~'
    }

    pub fn is_space(&self) -> bool {
        self.0 == b' '
    }
}