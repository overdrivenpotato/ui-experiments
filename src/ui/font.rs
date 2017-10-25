use ::Color;

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
    Bold,
    Italic,
    BoldItalic,
}

#[derive(Debug, Clone)]
pub struct Font {
    pub family: String,
    pub weight: Weight,
    pub style: Style,
    pub color: Color,
}

impl Default for Font {
    fn default() -> Self {
        Font {
            family: String::from("Arial"),
            weight: Weight::Regular,
            style: Style::Regular,
            color: Color::black(),
        }
    }
}
