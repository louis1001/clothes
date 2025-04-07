use crate::graphics::{
    canvas::Canvas,
    pixel::{RGBPixel, TwoBitPixel},
};

pub trait ImageSource {
    type Pixel;
    fn width(&self) -> usize;
    fn height(&self) -> usize;

    fn pixel_at(&self, x: usize, y: usize) -> Option<&Self::Pixel>;
}

pub trait PpmOutput: ImageSource {
    fn header(&self) -> String;
    fn pixel_row(&self, y: usize) -> String;

    fn output(&self) -> String {
        let mut contents = String::new();

        contents.push_str(&self.header());
        contents.push('\n');

        for y in 0..self.height() {
            contents.push_str(&self.pixel_row(y));
            contents.push('\n');
        }

        contents
    }
}

// Canvas implementations
impl<Content: Default + Clone> ImageSource for Canvas<Content> {
    type Pixel = Content;

    fn width(&self) -> usize {
        self.size.width
    }

    fn height(&self) -> usize {
        self.size.height
    }

    fn pixel_at(&self, x: usize, y: usize) -> Option<&Self::Pixel> {
        self.get_at(x, y)
    }
}

impl PpmOutput for Canvas<TwoBitPixel> {
    fn header(&self) -> String {
        format!("P2\n{} {}\n1", self.width(), self.height())
    }

    fn pixel_row(&self, y: usize) -> String {
        let mut output = String::new();

        for x in 0..self.width() {
            let pixel = self.get_at(x, y).unwrap_or(&TwoBitPixel(false));

            let value = match pixel {
                TwoBitPixel(true) => '1',
                TwoBitPixel(false) => '0',
            };

            output.push(value);
            output.push(' ');
        }

        output
    }
}

impl PpmOutput for Canvas<RGBPixel> {
    fn header(&self) -> String {
        format!(
            "P3\n{} {}\n{}",
            self.width(),
            self.height(),
            self.max_value()
        )
    }

    fn pixel_row(&self, y: usize) -> String {
        let mut output = String::new();

        for x in 0..self.width() {
            let pixel = self.get_at(x, y).cloned().unwrap_or_default();

            let r = pixel.r();
            let g = pixel.g();
            let b = pixel.b();

            output.push_str(&format!("{r} {g} {b}"));

            output.push_str("   ");
        }

        output
    }
}

impl Canvas<RGBPixel> {
    fn max_value(&self) -> usize {
        255
    }
}
