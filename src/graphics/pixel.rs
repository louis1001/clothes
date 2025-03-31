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
    pub fn brightness(&self) -> f64 {
        0.299 * self.r() + 0.587 * self.g() + 0.114 * self.b()
    }
}

impl RGBPixel {
    pub fn white() -> Self {
        RGBPixel { r: 1., g: 0.98, b: 0.96 }
    }

    pub fn black() -> Self {
        RGBPixel { r: 0.3, g: 0.3, b: 0.45 }
    }
    
    pub fn red() -> Self {
        RGBPixel { r: 0.94, g: 0.4, b: 0.1 }
    }

    pub fn green() -> Self {
        RGBPixel { r: 0.4, g: 0.68, b: 0.2 }
    }

    pub fn blue() -> Self {
        RGBPixel { r: 0.4, g: 0.35, b: 0.98 }
    }
}

impl Default for RGBPixel {
    fn default() -> Self {
        Self::black()
    }
}