#![feature(conservative_impl_trait, never_type)]
#![recursion_limit="128"]

#[macro_use] extern crate stdweb;

mod ui;
mod blocks;
#[cfg(target_os = "emscripten")]
mod web;
mod events;

use ui::*;
use ui::position::Position;
use ui::font::Font;
use ui::border::Border;
use events::{Coordinates, Events};
use blocks::{block, Block, Data};

pub trait State: Default {
    type Message;

    fn reduce(self, message: Self::Message) -> Self;
}

impl State for () {
    type Message = !;

    fn reduce(self, _message: !) -> () { () }
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

fn sub_block() -> impl Block<Message = !> {
    let style = Style {
        font: Font {
            family: font::Family::Name(String::from("serif")),
            color: Color::green(),
            .. Font::default()
        },
        border: Border {
            width: Length(1.0),
            color: Color::green(),
            .. Border::default()
        },
        .. Style::default()
    };

    let _events = Events::new()
        .down(|Coordinates { x, y }, button| {
            println!("Button {:?} down in sub block at {}, {}", button, x, y);
        })
        .up(|Coordinates { x, y }, button| {
            println!("Button {:?} up in sub block at {}, {}", button, x, y);
        })
        .click(|Coordinates { x, y }| {
            println!("Mouse clicked in sub block at {}, {}", x, y);
        });

    block(Data::with(style, Events::new()), (
        "Sub",
        "Block",
    ))
}

pub fn test(_state: &()) -> impl Block<Message = !> {
    let style = Style {
        position: Position::Anchor,
        font: Font {
            family: font::Family::Name(String::from("sans-serif")),
            weight: font::Weight::Regular,
            style: font::Style::Italic,
            color: Color::black(),
        },
        border: Border {
            color: Color::black(),
            width: Length(2.0),
            .. Border::default()
        },
        .. Style::default()
    };

    let _events = Events::new()
        .click(|Coordinates { x, y }| println!("Mouse clicked at {}, {}", x, y))
        .down(|Coordinates { x, y }, button| {
            println!("Button {:?} down at {}, {}", button, x, y);
        })
        .up(|Coordinates { x, y }, button| {
            println!("Button {:?} up at {}, {}", button, x, y);
        });

    block(Data::with(style, Events::new()), (
        "Testing",
        "123",
        "456",
        sub_block(),
    ))
}

fn main() {
    #[cfg(target_os = "emscripten")]
    web::launch("root", test);
}
