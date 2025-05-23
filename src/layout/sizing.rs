#[derive(Clone, Debug)]
pub enum Sizing {
    Greedy(usize),
    Static(usize),
    Flexible(usize)
}

impl Sizing {
    pub fn clamped_accumulate_constrained(&mut self, other: &Sizing, constraint: usize) {
        *self = match self {
            Sizing::Static(n) => {
                let mut result = other.clone();
                result.clamped_add_constrained(*n, constraint);

                result
            }
            Sizing::Greedy(n) => {
                Sizing::Greedy(*n + other.min_content_size())
            }
            Sizing::Flexible(n) => {
                Sizing::Flexible(*n + other.min_content_size())
            }
        }
    }

    pub fn clamped_add(&mut self, n: usize) {
        self.clamped_add_constrained(n, usize::MAX)
    }

    pub fn clamped_add_constrained(&mut self, n: usize, constraint: usize) {
        match self {
            Sizing::Static(sz) | Sizing::Greedy(sz) | Sizing::Flexible(sz) => {
                *sz = sz.checked_add(n).unwrap_or(*sz).min(constraint);
            }
        };
    }

    pub fn min_content_size(&self) -> usize {
        match self {
            Sizing::Static(sz) | Sizing::Greedy(sz) | Sizing::Flexible(sz) => *sz
        }
    }
}

#[derive(Clone, Debug)]
pub struct ItemSizing {
    pub horizontal: Sizing,
    pub vertical: Sizing
}

impl ItemSizing {
    pub fn new(horizontal: Sizing, vertical: Sizing) -> Self {
        ItemSizing { horizontal, vertical }
    }

    pub fn fit_into(&self, bounds: &super::geometry::Rect) -> super::geometry::Rect {
        let width = match self.horizontal {
            Sizing::Greedy(n) | Sizing::Flexible(n) => bounds.width.max(n),
            Sizing::Static(n) => n
        };

        let height = match self.vertical {
            Sizing::Greedy(n) | Sizing::Flexible(n) => bounds.height.max(n),
            Sizing::Static(n) => n
        };

        super::geometry::Rect::new(
            bounds.x,
            bounds.y,
            width,
            height
        )
    }
}