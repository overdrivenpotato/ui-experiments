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

// Enums should be re-exported.
use self::position::Position;
use self::size::Size;
use self::content::Content;
use self::spacing::Spacing;
use self::border::Border;
pub use self::background::Background;
use self::shadow::Shadow;
use self::render::Render;
use self::reactive::Reactive;
use self::font::Font;

pub use self::reactive::Cursor;

#[derive(Debug, Default, Copy, Clone)]
pub struct Color(u8, u8, u8, u8);

impl Color {
    pub fn get_rgba(&self) -> (u8, u8, u8, u8) {
        (self.0, self.1, self.2, self.3)
    }

    pub fn rgba(r: u8, g: u8, b: u8, a: u8) -> Self {
        Color(r, g, b, a)
    }

    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        Color(r, g, b, 255)
    }

    pub fn white() -> Self {
        Self::rgb(255, 255, 255)
    }

    pub fn black() -> Self {
        Self::rgb(0, 0, 0)
    }

    pub fn green() -> Self {
        Self::rgb(0, 128, 0)
    }

    pub fn red() -> Self {
        Self::rgb(128, 0, 0)
    }
}

macro_rules! from_primitive_number {
    ($t:ident, ($target:ty), $($p:ty),+) => {
        $(
            impl From<$p> for $t {
                fn from(p: $p) -> Self {
                    $t(p as $target)
                }
            }
        )*
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

from_primitive_number!(Length, (f32), u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64);
from_primitive_number!(Percentage, (f32), u8, u16, u32, u64, i8, i16, i32, i64, usize, isize, f32, f64);

#[derive(Debug, Clone, Copy)]
pub enum EdgeMode {
    Inset,
    Outset,
}

impl Default for EdgeMode {
    fn default() -> Self {
        EdgeMode::Outset
    }
}

#[derive(Debug, Default, Clone, Copy)]
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

impl<T> Into<(T, T, T, T)> for Quadruple<T> {
    fn into(self) -> (T, T, T, T) {
        (self.a, self.b, self.c, self.d)
    }
}

impl<T> From<(T, T, T, T)> for Quadruple<T> {
    fn from((a, b, c, d): (T, T, T, T)) -> Self {
        Quadruple { a, b, c, d }
    }
}

#[derive(Debug, Clone, Copy)]
pub enum Unit {
    Length(Length),
    Percentage(Percentage),
}

impl Unit {
    pub fn percentage(p: f32) -> Unit {
        Unit::Percentage(Percentage(p))
    }

    pub fn spx<T>(spx: T) -> Unit where T: Into<f32> {
        Unit::Length(Length(spx.into()))
    }
}

impl Default for Unit {
    fn default() -> Self {
        Unit::Length(Length(0.0))
    }
}

#[derive(Default, Debug, Clone)]
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

impl Style {
    pub fn new<F>(f: F) -> Style where F: Fn(&mut Style) {
        let mut s = Default::default();
        f(&mut s);
        s
    }
}
