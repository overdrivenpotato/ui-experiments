use std::marker::PhantomData;

use ui::Style;
use events::{self, DefaultEvents, EventHandler};

pub struct Build<E> {
    pub style: Style,
    pub event_handler: E,
}

impl<T> Build<DefaultEvents<T>> {
    pub fn new() -> Self {
        Self {
            style: Default::default(),
            event_handler: DefaultEvents::new(),
        }
    }
}

impl<E> Build<E> {
    pub fn with(style: Style, event_handler: E) -> Self {
        Self { style, event_handler }
    }

    pub fn style(self, style: Style) -> Self {
        Self { style, ..self }
    }

    pub fn events<H>(self, handler: H) -> Build<H> {
        Build {
            style: self.style,
            event_handler: handler,
        }
    }

    pub fn block<C>(self, child: C) -> impl Block<Message = E::Message>
    where
        C: Child<E::Message>,
        E: EventHandler,
    {
        BlockData { data: self, child }
    }
}

pub trait Walker {
    type Message;
    type Walked;

    fn group<M, G>(self, G) -> Self::Walked
    where
        G: Group<M>,
        Self::Message: From<M>;

    fn block<E, M, C>(self, Build<E>, C) -> Self::Walked
    where
        E: EventHandler<Message = M>,
        C: Child<M>,
        Self::Message: From<M>,
        E: 'static,
        M: 'static;

    fn text(self, text: &'static str) -> Self::Walked;

    fn empty(self) -> Self::Walked;
}

pub trait Child<M> {
    fn walk<T>(self, T) -> T::Walked where T: Walker, T::Message: From<M>;
}

impl<M> Child<M> for () {
    fn walk<T>(self, walker: T) -> T::Walked where T: Walker, T::Message: From<M> {
        walker.empty()
    }
}

impl<M> Child<M> for &'static str {
    fn walk<T>(self, walker: T) -> T::Walked where T: Walker, T::Message: From<M> {
        walker.text(self)
    }
}

macro_rules! impl_child_tuple {
    ($(($Reference:ident $($T:ident $idx:tt),*)),*,) => {$(
        impl<_M, $Reference, $($T),*> Child<_M> for ($Reference, $($T),*)
        where
            Self: Group<_M>,
        {
            fn walk<_T>(self, walker: _T) -> _T::Walked where _T: Walker, _T::Message: From<_M> {
                walker.group(self)
            }
        }

        impl<_M, $Reference, $($T),*> Group<_M> for ($Reference, $($T),*)
        where
            _M: 'static,
            $Reference: Child<_M>,
            $($T: Child<_M>),*
        {
            fn consolidate<_C>(self, mut consolidator: _C)
            where
                _C: Consolidator,
                _C::Message: From<_M>,
            {
                consolidator.child(self.0);
                $(consolidator.child(self.$idx);)*
            }
        }
    )*}
}

impl_child_tuple! {
    (A),
    (A B 1),
    (A B 1, C 2),
    (A B 1, C 2, D 3),
    (A B 1, C 2, D 3, E 4),
    (A B 1, C 2, D 3, E 4, F 5),
    (A B 1, C 2, D 3, E 4, F 5, G 6),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24),
    (A B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25),
}

pub trait Consolidator {
    type Message;

    fn child<M, C>(&mut self, C) where C: Child<M>, Self::Message: From<M>, M: 'static;
}

struct ProxyConsolidate<C, MI, MT> {
    consolidator: C,
    _input: PhantomData<MI>,
    _target: PhantomData<MT>,
}

impl<C, MI, MT> ProxyConsolidate<C, MI, MT> {
    fn new(consolidator: C) -> Self {
        Self {
            consolidator,
            _input: PhantomData,
            _target: PhantomData,
        }
    }
}

impl<_C, MI, MT> Consolidator for ProxyConsolidate<_C, MI, MT>
where
    _C: Consolidator<Message = MT>,
    MT: From<MI>,
    MI: 'static,
{
    type Message = MI;

    fn child<M, C>(&mut self, child: C) where C: Child<M>, Self::Message: From<M>, M: 'static {
        self.consolidator.child(ChildUpgrade::new(child));
    }
}

struct GroupUpgrade<G, M> {
    group: G,
    _message: PhantomData<M>,
}

impl<G, M> GroupUpgrade<G, M> {
    fn new(group: G) -> Self {
        Self { group, _message: PhantomData }
    }
}

impl<G, MI, MT> Group<MT> for GroupUpgrade<G, MI>
where
    G: Group<MI>,
    MT: From<MI>,
    MT: 'static,
{
    fn consolidate<C>(self, consolidator: C)
    where
        C: Consolidator,
        C::Message: From<MT>
    {
        self.group.consolidate(ProxyConsolidate::new(consolidator));
    }
}

pub trait Group<M> {
    fn consolidate<C>(self, C)
    where
        C: Consolidator,
        C::Message: From<M>;
}

pub trait Block {
    type Message;
    type EventHandler: EventHandler<Message = Self::Message>;
    type Child: Child<Self::Message>;

    fn extract(self) -> BlockData<Self::EventHandler, Self::Child>;
}

pub struct BlockData<E, C> {
    data: Build<E>,
    child: C,
}

impl<E, C> Block for BlockData<E, C>
where
    E: EventHandler,
    C: Child<E::Message>,
{
    type Message = E::Message;
    type EventHandler = E;
    type Child = C;

    fn extract(self) -> BlockData<Self::EventHandler, Self::Child> {
        let BlockData { data, child } = self;

        BlockData { data, child }
    }
}

struct ChildUpgrade<C, MI, MT> {
    child: C,
    _input: PhantomData<MI>,
    _target: PhantomData<MT>
}

impl<C, MI, MT> ChildUpgrade<C, MI, MT> where C: Child<MI> {
    fn new(child: C) -> Self {
        Self {
            child,
            _input: PhantomData,
            _target: PhantomData,
        }
    }
}

struct ProxyWalk<T, M> {
    walker: T,
    _message: PhantomData<M>,
}

impl<T, M> ProxyWalk<T, M> {
    fn new(walker: T) -> Self {
        Self {
            walker,
            _message: PhantomData,
        }
    }
}

impl<T, MI> Walker for ProxyWalk<T, MI>
where
    T: Walker,
    T::Message: From<MI>,
    MI: 'static,
{
    type Walked = T::Walked;
    type Message = MI;

    fn group<M, G>(self, group: G) -> Self::Walked
    where
        G: Group<M>,
        Self::Message: From<M>
    {
        self.walker.group(GroupUpgrade::new(group))
    }

    fn block<E, M, C>(self, data: Build<E>, child: C) -> Self::Walked
    where
        E: EventHandler<Message = M>,
        C: Child<M>,
        Self::Message: From<M>,
        E: 'static,
    {
        let data = Build::with(data.style, events::Upgrade::new(data.event_handler));
        let child = ChildUpgrade::new(child);

        self.walker.block(data, child)
    }

    fn text(self, text: &'static str) -> Self::Walked {
        self.walker.text(text)
    }

    fn empty(self) -> Self::Walked {
        self.walker.empty()
    }
}


impl<C, MI, MT> Child<MT> for ChildUpgrade<C, MI, MT>
where
    C: Child<MI>,
    MT: From<MI>,
    MT: 'static,
{
    fn walk<T>(self, walker: T) -> T::Walked where T: Walker, T::Message: From<MT> {
        self.child.walk(ProxyWalk::new(walker))
    }
}

impl<B, M> Child<M> for B
where
    B: Block,
    B::EventHandler: 'static,
    M: From<B::Message>,
    M: 'static,
{
    fn walk<T>(self, walker: T) -> T::Walked
    where
        T: Walker,
        T::Message: From<M>
    {
        let BlockData { data, child, } = self.extract();
        let Build { style, event_handler } = data;

        // TODO: This code is duplicated?..
        // Upgrade contents with message wrapper.
        let data = Build::with(style, events::Upgrade::new(event_handler));
        let child = ChildUpgrade::new(child);

        walker.block(data, child)
    }
}
