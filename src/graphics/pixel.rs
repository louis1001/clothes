#[derive(Default, Clone, Copy, Debug, PartialEq)]
pub enum TwoBitPixel {
    #[default]
    Zero,
    One
}

impl From<bool> for TwoBitPixel {
    fn from(value: bool) -> Self {
        if value {
            TwoBitPixel::One
        } else {
            TwoBitPixel::Zero
        }
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct RGBPixel {
    r: u8,
    g: u8,
    b: u8
}

impl RGBPixel {
    pub fn r(&self) -> u8 {
        self.r
    }

    pub fn g(&self) -> u8 {
        self.g
    }

    pub fn b(&self) -> u8 {
        self.b
    }
}

impl RGBPixel {
    pub fn brightness(&self) -> f64 {
        0.299 * (self.r() as f64 / 255.0) + 0.587 * (self.g() as f64 / 255.0) + 0.114 * (self.b() as f64 / 255.0)
    }
}

impl RGBPixel {
    pub fn white() -> Self {
        RGBPixel { r: 0xff, g: 0xfa, b: 0xf6 }
    }

    pub fn black() -> Self {
        RGBPixel { r: 0x4c, g: 0x4c, b: 0x73 }
    }
    
    pub fn red() -> Self {
        RGBPixel { r: 0xf0, g: 0x65, b: 0x19 }
    }

    pub fn green() -> Self {
        RGBPixel { r: 0x66, g: 0xae, b: 0x33 }
    }

    pub fn blue() -> Self {
        RGBPixel { r: 0x66, g: 0x59, b: 0xfa }
    }
}

impl Default for RGBPixel {
    fn default() -> Self {
        Self::black()
    }
}