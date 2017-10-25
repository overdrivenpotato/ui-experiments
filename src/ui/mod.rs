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
pub struct Color(f32, f32, f32, f32);

impl Color {
    pub fn black() -> Color {
        Color(0.0, 1.0, 0.0, 1.0)
    }

    pub fn green() -> Color {
        Color(0.0, 1.0, 0.0, 1.0)
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

#[derive(Debug, Default)]
pub struct Quadruple<T> {
    a: T,
    b: T,
    c: T,
    d: T,
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
