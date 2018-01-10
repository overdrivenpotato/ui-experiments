#![feature(conservative_impl_trait)]

extern crate blocks;

use blocks::{ui, Block, Reactor, Build, Events};

struct State {
    number: i32,
}

#[derive(Copy, Clone)]
enum Message {
    Add,
    Subtract,
}

impl blocks::State for State {
    type Message = Message;

    fn new(_: Reactor<Self::Message>) -> Self {
        Self {
            number: 0
        }
    }

    fn reduce(&mut self, message: Self::Message) {
        match message {
            Message::Add => self.number += 1,
            Message::Subtract => self.number -= 1,
        }
    }
}

/// Create a button.
fn make_button(text: &'static str, message: Message) -> impl Block<Message = Message> {
    let style = ui::Style::new(move |s| {
        s.font.color(ui::Color::white());
        s.background.color(ui::Color::black());
        s.size.width.target(ui::Unit::spx(100.0));
        s.reactive.cursor(ui::Cursor::Pointer);
    });

    let events = Events::new()
        .mouse_down(move |_, _| message);

    Build::with(style, events).block(text)
}

fn app(state: &State) -> impl Block<Message = Message> {
    Build::new().block((
        format!("Number: {}", state.number),
        make_button("Add 1 to number", Message::Add),
        make_button("Subtract 1 from number", Message::Subtract),
    ))
}

fn main() {
    blocks::web::launch(app);
}
