use ui::{font, Color, Style};

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

impl Inline for Style {
    fn inline(&self) -> String {
        [
            self.font.inline(),
        ].join(";")
    }
}
