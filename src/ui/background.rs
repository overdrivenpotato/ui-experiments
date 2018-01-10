use super::{Double, Color, Unit};

#[derive(Debug, Clone, Copy)]
pub enum Size {
    Scale(Double<Unit>),
    Cover,
    Contain,
}

impl Default for Size {
    fn default() -> Self {
        Size::Cover
    }
}

#[derive(Debug, Clone)]
pub enum Background {
    Color(Color),
    Image(Vec<u8>, Size),
}

impl Default for Background {
    fn default() -> Self {
        Background::Color(Default::default())
    }
}

impl Background {
    pub fn color(&mut self, color: Color) {
        *self = Background::Color(color);
    }

    pub fn image(&mut self, image: Vec<u8>, size: Size) {
        *self = Background::Image(image, size);
    }
}
