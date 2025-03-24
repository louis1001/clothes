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
    r: f64,
    g: f64,
    b: f64
}

impl RGBPixel {
    pub fn r(&self) -> f64 {
        self.r
    }

    pub fn g(&self) -> f64 {
        self.g
    }

    pub fn b(&self) -> f64 {
        self.b
    }
}

impl RGBPixel {
    pub fn white() -> Self {
        RGBPixel { r: 1., g: 1., b: 1. }
    }

    pub fn black() -> Self {
        RGBPixel { r: 0., g: 0., b: 0. }
    }
    
    pub fn red() -> Self {
        RGBPixel { r: 1., g: 0., b: 0. }
    }

    pub fn green() -> Self {
        RGBPixel { r: 0., g: 1., b: 0. }
    }

    pub fn blue() -> Self {
        RGBPixel { r: 0., g: 0., b: 1. }
    }
}

impl Default for RGBPixel {
    fn default() -> Self {
        Self::black()
    }
}