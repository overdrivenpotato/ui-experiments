use stdweb::web::{self, Element, INode};

use ui::{font, Quadruple, EdgeMode, Color, Length, Style};
use ui::border::Border;
use ui::spacing::Spacing;

pub trait SetStyle {
    fn set_style(&mut self, style: Style);
}

impl SetStyle for Element {
    fn set_style(&mut self, style: Style) {
        js! {
            @{&*self}.setAttribute("style", @{style.inline()})
        }
    }
}

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
        format!("{}px", self.0)
    }
}

impl<T> Inline for Quadruple<T> where T: Inline, T: Copy {
    fn inline(&self) -> String {
        let (a, b, c, d) = self.clone().to_tuple();

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

impl Inline for Style {
    fn inline(&self) -> String {
        [
            self.spacing.inline(),
            self.font.inline(),
            self.border.inline(),
        ].join(";")
    }
}

fn class(name: &'static str, style: Style) -> String {
    format!("{}{{{}}}", name, style.inline())
}

pub fn inject() {
    if let Some(head) = web::document().query_selector("head") {
        let override_style = class("*", Default::default());
        let style = web::document().create_element("style");

        style.set_text_content(&override_style);
        head.append_child(&style);
    } else {
        println!("Failed to inject styles.");
    }
}
