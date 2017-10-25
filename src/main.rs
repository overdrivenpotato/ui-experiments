#![feature(conservative_impl_trait, never_type)]

#[macro_use] extern crate stdweb;

mod ui;
mod blocks;
#[cfg(target_os = "emscripten")]
mod web;
mod events;

use ui::*;
use ui::position::Position;
use ui::font::Font;
use events::{Button, Coordinates, Events};
use blocks::{block, Block, Data};

fn sub_block() -> impl Block<Message = ()> {
    let style = Style {
        font: Font {
            family: font::Family::Name(String::from("serif")),
            color: Color::green(),
            .. Font::default()
        },
        .. Style::default()
    };

    let events = Events::new()
        .click(|Coordinates { x, y }| {
            println!("Mouse clicked in sub block at {}, {}", x, y);
        });

    block(Data::with(style, events), (
        "Sub",
        "Block",
    ))
}

pub fn test() -> impl Block<Message = ()> {
    let style = Style {
        position: Position::Anchor,
        font: Font {
            family: font::Family::Name(String::from("sans-serif")),
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
        "Testing",
        "123",
        "456",
        sub_block(),
    ))
}

fn main() {
    #[cfg(target_os = "emscripten")]
    web::launch("root", test());
}
