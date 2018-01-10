#![feature(conservative_impl_trait)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

pub mod ui;
pub mod block;
pub mod events;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub mod web;

use std::marker::PhantomData;

pub use block::{Block, Build};
pub use events::Events;

#[derive(Copy, Clone)]
pub struct Reactor<M> {
    _message: PhantomData<M>,
}

impl<M> Reactor<M> {
    pub fn new() -> Self {
        Self {
            _message: PhantomData
        }
    }
}

pub trait State: Send + 'static {
    type Message: 'static + Send;

    fn new(Reactor<Self::Message>) -> Self;
    fn reduce(&mut self, Self::Message);
}
