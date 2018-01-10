use super::Color;

#[derive(Debug, Clone)]
pub enum Weight {
    ExtraLight,
    Light,
    Regular,
    Bold,
    ExtraBold,
}

#[derive(Debug, Clone)]
pub enum Style {
    Regular,
    Italic,
}

#[derive(Debug, Clone)]
pub enum Family {
    Inherit,
    Name(String),
}

#[derive(Debug, Clone)]
pub struct Font {
    pub family: Family,
    pub weight: Weight,
    pub style: Style,
    pub color: Color,
}

impl Default for Font {
    fn default() -> Self {
        Font {
            family: Family::Inherit,
            weight: Weight::Regular,
            style: Style::Regular,
            color: Color::black(),
        }
    }
}

impl Font {
    pub fn color(&mut self, color: Color) {
        self.color = color;
    }
}
