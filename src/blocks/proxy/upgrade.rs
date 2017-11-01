use std::marker::PhantomData;

use blocks::{self, Consolidator, Walker};

/// Upgrade a group to send target messages.
pub struct Group<G, M> {
    group: G,
    _message: PhantomData<M>,
}

impl<G, M> Group<G, M> {
    pub fn new(group: G) -> Self {
        Self { group, _message: PhantomData }
    }
}

impl<G, MI, MT> blocks::Group<MT> for Group<G, MI>
where
    G: blocks::Group<MI>,
    MT: From<MI>,
    MT: 'static,
{
    fn consolidate<C>(self, consolidator: C)
    where
        C: Consolidator,
        C::Message: From<MT>
    {
        self.group.consolidate(super::Consolidate::new(consolidator));
    }
}

/// Upgrade a child to send target messages.
pub struct Child<C, MI, MT> {
    child: C,
    _input: PhantomData<MI>,
    _target: PhantomData<MT>
}

impl<C, MI, MT> Child<C, MI, MT> where C: blocks::Child<MI> {
    pub fn new(child: C) -> Self {
        Self {
            child,
            _input: PhantomData,
            _target: PhantomData,
        }
    }
}

impl<C, MI, MT> blocks::Child<MT> for Child<C, MI, MT>
where
    C: blocks::Child<MI>,
    MT: From<MI>,
    MT: 'static,
{
    fn walk<T>(self, walker: T) -> T::Walked where T: Walker, T::Message: From<MT> {
        self.child.walk(super::Walk::new(walker))
    }
}
