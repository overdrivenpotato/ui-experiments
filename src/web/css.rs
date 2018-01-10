use ui::{self, font, Quadruple, EdgeMode, Color, Length, Style};
use ui::border::Border;
use ui::spacing::Spacing;

use super::ffi;

#[derive(Clone)]
pub struct Css {
    rendered: Vec<String>,
}

impl Css {
    fn new() -> Self {
        Css {
            rendered: Vec::new()
        }
    }

    fn property<A>(&mut self, key: &'static str, value: A) where A: AsRef<str> {
        self.rendered.push(format!("{}:{}", key, value.as_ref()));
    }

    fn render(self) -> String {
        self.rendered.join(";")
    }
}

impl Inline for Css {
    fn inline(&self) -> String {
        self.clone().render()
    }
}

pub trait Inline {
    fn inline(&self) -> String;
}

impl Inline for Color {
    fn inline(&self) -> String {
        let (r, g, b, a) = self.get_rgba();

        if a != 255 {
            format!("rgba({},{},{},{})",
                r,
                g,
                b,
                a as f32 / 255.0
            )
        } else {
            format!("rgb({},{},{})", r, g, b)
        }
    }
}

impl Inline for Length {
    fn inline(&self) -> String {
        format!("{}em", self.0 / 10.0)
    }
}

impl<T> Inline for Quadruple<T> where T: Inline + Clone {
    fn inline(&self) -> String {
        let (a, b, c, d) = self.clone().into();

        format!("{} {} {} {}",
            a.inline(),
            b.inline(),
            c.inline(),
            d.inline()
        )
    }
}

impl Inline for font::Font {
    fn inline(&self) -> String {
        let mut css = Css::new();

        css.property("font-family", match self.family {
            font::Family::Inherit => String::from("inherit"),
            font::Family::Name(ref name) => name.clone(),
        });

        css.property("font-weight", match self.weight {
            font::Weight::ExtraLight => "100",
            font::Weight::Light => "300",
            font::Weight::Regular => "500",
            font::Weight::Bold => "700",
            font::Weight::ExtraBold => "900",
        });

        css.property("font-style", match self.style {
            font::Style::Regular => "normal",
            font::Style::Italic => "italic",
        });

        css.property("color", self.color.inline());

        css.render()
    }
}

impl Inline for Border {
    fn inline(&self) -> String {
        let mut css = Css::new();

        css.property("border-color", self.color.inline());
        css.property("border-width", self.width.inline());
        css.property("box-sizing", match self.mode {
            EdgeMode::Inset => "border-box",
            EdgeMode::Outset => "content-box",
        });
        css.property("border-style", String::from("solid"));

        css.render()
    }
}

impl Inline for Spacing {
    fn inline(&self) -> String {
        let mut css = Css::new();

        css.property("padding", self.inner.inline());
        css.property("margin", self.outer.inline());

        css.render()
    }
}

impl Inline for ui::Background {
    fn inline(&self) -> String {
        let mut css = Css::new();

        match *self {
            ui::Background::Color(ref color) => {
                css.property("background-color", color.inline());
            }

            _ => {}
        }

        css.render()
    }
}

impl Inline for ui::reactive::Reactive {
    fn inline(&self) -> String {
        let mut css = Css::new();

        match self.cursor {
            ui::Cursor::Pointer => {
                css.property("cursor", "pointer");
            }
            _ => {}
        }

        css.render()
    }
}

impl Inline for Style {
    fn inline(&self) -> String {
        let mut user_select = Css::new();

        user_select.property("-moz-user-select", "none");
        user_select.property("-webkit-user-select", "none");
        user_select.property("user-select", "none");

        [
            self.spacing.inline(),
            self.font.inline(),
            self.border.inline(),
            self.background.inline(),
            self.reactive.inline(),
            user_select.render(),
        ].join(";")
    }
}

fn class<T>(name: &'static str, properties: T) -> String where T: Inline {
    format!("{}{{{}}}", name, properties.inline())
}

pub fn inject() {
    let base = class("*", Style::default());

    let mut container = Css::new();
    container.property("height", "100%");
    container.property("display", "flex");
    container.property("flex-direction", "column");

    let container = class("html,body", container);

    ffi::inject_stylesheet(format!("{}{}", base, container));
}
