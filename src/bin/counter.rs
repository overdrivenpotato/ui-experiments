#![feature(conservative_impl_trait)]

extern crate blocks;

use blocks::ui::{Color, Length};
use blocks::ui::border::Border;
use blocks::{Block, Build, Style, Events};

struct State {
    clicks: i32,
}

impl Default for State {
    fn default() -> Self {
        Self {
            clicks: 0
        }
    }
}

enum Message {
    Click(Click),
}

#[derive(Copy, Clone)]
enum Click {
    Up,
    Down,
}

impl From<Click> for Message {
    fn from(click: Click) -> Self {
        Message::Click(click)
    }
}

impl blocks::State for State {
    type Message = Message;

    fn reduce(self, message: Self::Message) -> Self {
        use Message::*;
        use Click;

        match message {
            Click(Click::Up) => State {
                clicks: self.clicks + 1,
            },
            Click(Click::Down) => State {
                clicks: self.clicks - 1,
            },
        }
    }
}

fn color_change(message: Click, text: &'static str) -> impl Block<Message = Click> {
    let style = Style {
        border: Border {
            width: Length(1.0),
            color: Color::black(),
            .. Border::default()
        },
        .. Style::default()
    };

    let events = Events::new().click(move |_| message);

    Build::with(style, events).block(text)
}

fn app(state: &State) -> impl Block<Message = Message> {
    Build::new().block((
        format!("Clicks: {}", state.clicks),
        color_change(Click::Up, "Add 1"),
        color_change(Click::Down, "Subtract 1"),
    ))
}

fn main() {
    #[cfg(target_os = "emscripten")]
    blocks::web::launch("root", app);
}
