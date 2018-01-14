#![feature(conservative_impl_trait)]

extern crate serde;
extern crate serde_json;
#[macro_use] extern crate serde_derive;
#[macro_use] extern crate lazy_static;

pub mod ui;
pub mod block;
pub mod events;
mod reactor;

#[cfg(all(target_arch = "wasm32", not(target_os = "emscripten")))]
pub mod web;

pub use reactor::Reactor;
pub use block::{Block, Build};
pub use events::Events;

pub trait Update: Send + 'static {
    type Message: 'static + Send;

    fn reduce(&self, message: Self::Message);
    fn clone(&self) -> Box<Update<Message = Self::Message>>;
}

impl<M> Update for Box<Update<Message = M>> where M: Send + 'static {
    type Message = M;

    fn reduce(&self, message: Self::Message) {
        (**self).reduce(message);
    }

    fn clone(&self) -> Box<Update<Message = Self::Message>> {
        (**self).clone()
    }
}

pub trait State: Send + 'static {
    type Message: Send + 'static;

    fn new(Reactor<Self::Message>) -> Self;
    fn reduce(&mut self, Self::Message);
}
