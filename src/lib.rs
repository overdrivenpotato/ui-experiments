#![feature(conservative_impl_trait)]
#![recursion_limit="128"]

#[macro_use] extern crate stdweb;

pub mod ui;
pub mod block;
pub mod events;

pub use block::{Block, Build};
pub use events::Events;
pub use ui::Style;

pub trait State: Default {
    type Message;

    fn reduce(self, message: Self::Message) -> Self;
}

pub trait App<S, B>: Copy where B: Block, S: State<Message = B::Message> {
    fn render(&self, state: &S) -> B;
}

impl<F, S, B> App<S, B> for F
where
    B: Block,
    S: State<Message = B::Message>,
    F: Copy + Fn(&S) -> B,
{
    fn render(&self, state: &S) -> B {
        self(state)
    }
}
