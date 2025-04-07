use crate::graphics::pixel::{RGBPixel, TwoBitPixel};

use super::Canvas;

pub trait PixelConvertion<FromPixel> {}

pub struct CanvasConversion<'from, FromPixel: Clone + Default, ToPixel: Clone + Default> {
    source_canvas: &'from Canvas<FromPixel>,
    conversion: Box<dyn Fn(FromPixel) -> ToPixel>,
}

impl<'from, FromPixel: Clone + Default, ToPixel: Clone + Default>
    From<CanvasConversion<'from, FromPixel, ToPixel>> for Canvas<ToPixel>
{
    fn from(value: CanvasConversion<'from, FromPixel, ToPixel>) -> Self {
        let source_canvas = value.source_canvas;

        let new_pixels = source_canvas
            .contents
            .iter()
            .map(|x| (value.conversion)(x.clone()))
            .collect();

        Canvas::create_with_content(new_pixels, &source_canvas.size)
    }
}

impl Canvas<RGBPixel> {
    pub fn convert_to(&self, threshold: f64) -> CanvasConversion<RGBPixel, TwoBitPixel> {
        CanvasConversion {
            source_canvas: self,
            conversion: Box::new({
                let min_value = threshold;

                move |pixel| (pixel.brightness() > min_value).into()
            }),
        }
    }
}

impl From<TwoBitPixel> for RGBPixel {
    fn from(value: TwoBitPixel) -> Self {
        match value {
            TwoBitPixel(true) => RGBPixel::white(),
            TwoBitPixel(false) => RGBPixel::black(),
        }
    }
}

impl From<Canvas<RGBPixel>> for Canvas<TwoBitPixel> {
    fn from(value: Canvas<RGBPixel>) -> Self {
        value.convert_to(0.5).into()
    }
}

impl From<Canvas<TwoBitPixel>> for Canvas<RGBPixel> {
    fn from(value: Canvas<TwoBitPixel>) -> Self {
        let new_pixels = value.contents.iter().map(|x| (*x).into()).collect();

        Canvas::create_with_content(new_pixels, &value.size)
    }
}

#[test]
fn convert_rgb_canvas_to_twobit() {
    let mut rgb_canvas: Canvas<RGBPixel> = Canvas::create(5, 5);

    rgb_canvas.clear_with(&RGBPixel::white());

    let twobit_canvas: Canvas<TwoBitPixel> = rgb_canvas.convert_to(0.5).into();

    let new_rgb_canvas: Canvas<RGBPixel> = twobit_canvas.into();

    assert_eq!(rgb_canvas.contents, new_rgb_canvas.contents);
}
