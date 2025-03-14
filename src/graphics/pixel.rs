#[derive(Default, Clone)]
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

#[derive(Default, Clone)]
pub struct RGBPixel {
    r: f64,
    g: f64,
    b: f64
}