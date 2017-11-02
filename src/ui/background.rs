use super::{Double, Color, Unit};

#[derive(Debug, Clone, Copy)]
pub enum Size {
    DoubleUnit(Double<Unit>),
    Cover,
    Contain,
}

impl Default for Size {
    fn default() -> Self {
        Size::Cover
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Background {
    Color(Color),
    Image(Double<Unit>, Size),
}

impl Default for Background {
    fn default() -> Self {
        Background::Color(Default::default())
    }
}
