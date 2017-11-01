pub mod position;
pub mod size;
pub mod content;
pub mod spacing;
pub mod border;
pub mod background;
pub mod shadow;
pub mod render;
pub mod reactive;
pub mod font;

#[derive(Debug, Default, Copy, Clone)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }

    pub fn black() -> Color {
        Color(0, 0, 0, 255)
    }

    pub fn green() -> Color {
        Color(0, 128, 0, 255)
    }

    pub fn red() -> Color {
        Color(128, 0, 0, 255)
    }
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Length(pub f32);

#[derive(Debug, Copy, Clone)]
pub struct Percentage(pub f32);

impl Default for Percentage {
    fn default() -> Self {
        Percentage(100.0)
    }
}

#[derive(Debug)]
pub enum EdgeMode {
    Inset,
    Outset,
}

impl Default for EdgeMode {
    fn default() -> Self {
        EdgeMode::Outset
    }
}

#[derive(Debug, Default)]
pub struct Double<T> {
    a: T,
    b: T,
}

#[derive(Debug, Default, Copy, Clone)]
pub struct Quadruple<T> {
    a: T,
    b: T,
    c: T,
    d: T,
}

impl<T> Quadruple<T> {
    pub fn to_tuple(self) -> (T, T, T, T) {
        (self.a, self.b, self.c, self.d)
    }
}

#[derive(Debug, Copy, Clone)]
pub enum Unit {
    Length(Length),
    Percentage(Percentage),
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Length(Length(0.0))
    }
}

#[derive(Default, Debug)]
pub struct Style {
    pub position: position::Position,
    pub size: size::Size,
    pub content: content::Content,
    pub spacing: spacing::Spacing,
    pub border: border::Border,
    pub background: background::Background,
    pub shadow: shadow::Shadow,
    pub render: render::Render,
    pub reactive: reactive::Reactive,
    pub font: font::Font,
}
