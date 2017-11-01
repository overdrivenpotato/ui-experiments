#![feature(conservative_impl_trait, never_type)]
#![recursion_limit="128"]

#[macro_use] extern crate stdweb;

mod ui;
mod block;
#[cfg(target_os = "emscripten")]
mod web;
mod events;

use ui::*;
use ui::font::Font;
use events::Events;
use block::{Block, Build};

pub trait State: Default {
    type Message;

    fn reduce(self, message: Self::Message) -> Self;
}

pub trait App<S, B>: Copy where B: Block, S: State<Message = B::Message> {
    fn render(&self, state: &S) -> B;
}

impl<S, B, F> App<S, B> for F
where
    B: Block,
    S: State<Message = B::Message>,
    F: Fn(&S) -> B + Copy,
{
    fn render(&self, state: &S) -> B {
        self(state)
    }
}

pub enum TestState {
    Red,
    Green,
}

impl Default for TestState {
    fn default() -> Self {
        TestState::Green
    }
}

pub enum TestMessage {
    Change(ColorChange),
}

#[derive(Copy, Clone)]
pub enum ColorChange {
    Red,
    Green,
}

impl From<ColorChange> for TestMessage {
    fn from(target: ColorChange) -> Self {
        TestMessage::Change(target)
    }
}

impl State for TestState {
    type Message = TestMessage;

    fn reduce(self, message: Self::Message) -> Self {
        match message {
            TestMessage::Change(ColorChange::Red) => TestState::Red,
            TestMessage::Change(ColorChange::Green) => TestState::Green,
        }
    }
}

fn color_change(
    color: Color,
    target: ColorChange,
    text: &'static str
) -> impl Block<Message = ColorChange> {
    let style = Style {
        font: Font { color, .. Font::default() },
        .. Style::default()
    };

    let events = Events::new()
        .click(move |_| target);

    Build::with(style, events).block(text)
}

pub fn app(state: &TestState) -> impl Block<Message = TestMessage> {
    let color = match *state {
        TestState::Red => Color::red(),
        TestState::Green => Color::green(),
    };

    Build::new().block((
        (
            "A",
            "Sub",
            "Group",
        ),
        "Test Text",
        color_change(color, ColorChange::Green, "[Click me!] Make green"),
        color_change(color, ColorChange::Red, "[Click me!] Make red"),
    ))
}

fn main() {
    #[cfg(target_os = "emscripten")]
    web::launch("root", app);
}
