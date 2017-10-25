#![feature(conservative_impl_trait, never_type)]

extern crate stdweb;

mod ui;
mod blocks;
#[cfg(target_os = "emscripten")]
mod web;
mod events;

use ui::*;
use ui::position::Position;
use events::{Button, Coordinates, Events};
use blocks::{block, Block, Data};

pub fn test() -> impl Block<Message = ()> {
    let style = Style {
        position: Position::Anchor,
        font: font::Font {
            family: String::from("Arial"),
            weight: font::Weight::Regular,
            style: font::Style::Italic,
            color: Color::black(),
        },
        .. Style::default()
    };

    let events = Events::new()
        .click(|Coordinates { x, y }| println!("Mouse clicked at {}, {}", x, y))
        .up(|Coordinates { x, y }, button| {
            match button {
                Button::Left => println!("Left mouse up at {}, {}", x, y),
                _ => println!("Mouse up with another button at {}, {}", x, y),
            }
        });

    block(Data::with(style, events), (
        block(Data::default(), (
            "Sub",
            "Test",
        )),
        "Testing",
        "123",
        "456",
    ))
}

fn main() {
    #[cfg(target_os = "emscripten")]
    web::launch("root", test());
}
