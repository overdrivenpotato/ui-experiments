#![feature(conservative_impl_trait, never_type)]

#[macro_use] extern crate stdweb;

mod ui;
mod blocks;
#[cfg(target_os = "emscripten")]
mod web;
mod events;

use ui::*;

fn main() {
    #[cfg(target_os = "emscripten")]
    web::mount("#root").launch(blocks::test());
}
