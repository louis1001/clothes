use core::f64;

#[derive(Clone, Debug)]
pub struct Rect {
    pub x: i64,
    pub y: i64,
    pub width: usize,
    pub height: usize,
}

impl Rect {
    pub fn new(x: i64, y: i64, width: usize, height: usize) -> Self {
        Self {
            x,
            y,
            width,
            height,
        }
    }

    pub fn zero() -> Self {
        Rect {
            x: 0,
            y: 0,
            width: 0,
            height: 0,
        }
    }

    pub fn sized(width: usize, height: usize) -> Self {
        Self {
            x: 0,
            y: 0,
            width,
            height,
        }
    }

    pub fn from_size(size: &Size) -> Self {
        Self { x: 0, y: 0, width: size.width, height: size.height }
    }

    // Utilities
    pub fn max_x(&self) -> i64 {
        self.x + self.width as i64
    }

    pub fn max_y(&self) -> i64 {
        self.y + self.height as i64
    }

    pub fn mid_x(&self) -> i64 {
        self.x + ((self.max_x() - self.x) / 2)
    }

    pub fn mid_y(&self) -> i64 {
        self.y + ((self.max_y() - self.y) / 2)
    }

    pub fn mid_top(&self) -> Vector {
        Vector::new(self.mid_x(), self.y)
    }

    pub fn mid_bottom(&self) -> Vector {
        Vector::new(self.mid_x(), self.max_y())
    }

    pub fn mid_left(&self) -> Vector {
        Vector::new(self.x, self.mid_y())
    }

    pub fn mid_right(&self) -> Vector {
        Vector::new(self.max_x(), self.mid_y())
    }

    pub fn size(&self) -> Size {
        Size::new(self.width, self.height)
    }

    pub fn top_left(&self) -> Vector {
        Vector::new(self.x, self.y)
    }

    pub fn top_right(&self) -> Vector {
        Vector::new(self.max_x(), self.y)
    }

    pub fn bottom_left(&self) -> Vector {
        Vector::new(self.x, self.max_y())
    }

    pub fn bottom_right(&self) -> Vector {
        Vector::new(self.max_x(), self.max_y())
    }
}

impl Default for Rect {
    fn default() -> Self {
        Self::zero()
    }
}

#[derive(Clone, Debug)]
pub struct Vector {
    x: i64,
    y: i64
}

impl Vector {
    pub fn zero() -> Vector {
        Vector { x: 0, y: 0 }
    }
    
    pub fn new(x: i64, y: i64) -> Vector {
        Vector {
            x, y
        }
    }

    pub fn sub(vec1: &Vector, vec2: &Vector) -> Vector {
        Vector { x: vec1.x - vec2.x, y: vec1.y - vec2.y }
    }

    pub fn x(&self) -> i64 { self.x }
    pub fn y(&self) -> i64 { self.y }

    pub fn set_x(&mut self, value: i64) {
        self.x = value;
    }

    pub fn set_y(&mut self, value: i64) {
        self.y = value;
    }
}

impl Vector {
    pub fn magnitude(&self) -> f64 {
        ((self.x.pow(2) + self.y.pow(2)) as f64).sqrt()
    }
}

#[derive(Clone, PartialEq, Debug)]
pub struct Size {
    pub width: usize,
    pub height: usize
}

impl Size {
    pub fn new(width: usize, height: usize) -> Self {
        Size {
            width,
            height
        }
    }

    pub fn zero() -> Self {
        Size { width: 0, height: 0 }
    }

    pub fn to_vector(&self) -> Vector {
        Vector::new(self.width as i64, self.height as i64)
    }

    pub fn scaled(self, scale: usize) -> Size {
        Size { width: self.width * scale, height: self.height * scale }
    }
}

#[derive(Clone, PartialEq, PartialOrd)]
pub struct Matrix<Item: Clone> {
    shape: (usize, usize),
    data: Vec<Item>
}

impl<Item: Clone> Matrix<Item> {
    pub fn with_rows(data: &[Item], row_count: usize) -> Self {
        assert!(data.len() % row_count == 0, "Matrix must completely fill the grid");

        let col_count = data.len() / row_count;
        Matrix { shape: (col_count, row_count), data: data.iter().map(|x| (*x).clone()).collect() }
    }

    pub fn data(&self) -> &[Item] {
        &self.data
    }

    pub fn shape(&self) -> (usize, usize) {
        self.shape
    }

    pub fn get(&self, x: usize, y: usize) -> &Item {
        assert!(x < self.shape.0 && y < self.shape.1);

        let index = y * self.shape.1 + x;

        &self.data[index]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> &mut Item {
        assert!(x < self.shape.0 && y < self.shape.1);

        let index = y * self.shape.1 + x;

        &mut self.data[index]
    }
}

// TODO: This should be done in floating point later
#[derive(Clone, Debug)]
pub struct CornerRadius {
    pub top_left: usize,
    pub top_right: usize,
    pub bottom_left: usize,
    pub bottom_right: usize
}

impl CornerRadius {
    pub fn new(top_left: usize, top_right: usize, bottom_left: usize, bottom_right: usize) -> CornerRadius {
        CornerRadius { top_left, top_right, bottom_left, bottom_right }
    }

    pub fn all(radius: usize) -> CornerRadius {
        CornerRadius { top_left: radius, top_right: radius, bottom_left: radius, bottom_right: radius }
    }
}

#[derive(Clone, Debug)]
pub enum Shape {
    Rectangle,
    RoundedRectangle(CornerRadius),
    Ellipse,
    Capsule
}

impl Shape {
    pub fn rounded_rect(radius: usize) -> Shape {
        Shape::RoundedRectangle(CornerRadius::all(radius))
    }

    pub fn rounded_rect_with_corners(top_left: usize, top_right: usize, bottom_left: usize, bottom_right: usize) -> Shape {
        Shape::RoundedRectangle(CornerRadius::new(top_left, top_right, bottom_left, bottom_right))
    }
}