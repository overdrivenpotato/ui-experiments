use std::marker::PhantomData;

use events::{self, EventHandler};
use block::{Build, Child, Consolidator, Group, Walker};

mod upgrade;

/// A walker implementation that wraps another walker and upgrades all children.
pub struct Walk<T, M> {
    walker: T,
    _message: PhantomData<M>,
}

impl<T, M> Walk<T, M> {
    pub fn new(walker: T) -> Self {
        Self {
            walker,
            _message: PhantomData,
        }
    }
}

impl<T, MI> Walker for Walk<T, MI>
where
    T: Walker,
    T::Message: From<MI>,
    MI: 'static + Send,
{
    type Walked = T::Walked;
    type Message = MI;

    fn group<M, G>(self, group: G) -> Self::Walked
    where
        G: Group<M>,
        Self::Message: From<M>
    {
        self.walker.group(upgrade::Group::new(group))
    }

    fn block<E, M, C>(self, data: Build<E>, child: C) -> Self::Walked
    where
        E: EventHandler<Message = M>,
        C: Child<M>,
        M: 'static + Send,
        Self::Message: From<M>,
        E: 'static,
    {
        let data = Build::with(data.style, events::Upgrade::new(data.event_handler));
        let child = upgrade::Child::new(child);

        self.walker.block(data, child)
    }

    fn text(self, text: &str) -> Self::Walked {
        self.walker.text(text)
    }

    fn empty(self) -> Self::Walked {
        self.walker.empty()
    }
}

/// A consolidator that upgrades all children.
pub struct Consolidate<C, MI, MT> {
    consolidator: C,
    _input: PhantomData<MI>,
    _target: PhantomData<MT>,
}

impl<C, MI, MT> Consolidate<C, MI, MT> {
    pub fn new(consolidator: C) -> Self {
        Self {
            consolidator,
            _input: PhantomData,
            _target: PhantomData,
        }
    }
}

impl<_C, MI, MT> Consolidator for Consolidate<_C, MI, MT>
where
    _C: Consolidator<Message = MT>,
    MT: From<MI>,
    MI: 'static + Send,
{
    type Message = MI;

    fn child<M, C>(&mut self, child: C) where C: Child<M>, Self::Message: From<M>, M: 'static {
        self.consolidator.child(upgrade::Child::new(child));
    }
}
