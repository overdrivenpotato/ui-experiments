use ui::Style;
use events::{DefaultEvents, EventHandler};

// TODO: Make this private again.
pub mod proxy;

/// Block builder.
pub struct Build<E> {
    pub style: Style,
    pub event_handler: E,
}

impl<T> Build<DefaultEvents<T>> {
    /// Create a block builder.
    pub fn new() -> Self {
        Self {
            style: Default::default(),
            event_handler: DefaultEvents::new(),
        }
    }

    /// Create a styled block builder.
    pub fn styled(style: Style) -> Self {
        Self { style, .. Self::new() }
    }
}

impl<E> Build<E> {
    /// Create a styled block builder with an event handler.
    pub fn with(style: Style, event_handler: E) -> Self {
        Self { style, event_handler }
    }

    /// Create a block from this builder.
    pub fn block<C>(self, child: C) -> impl Block<Message = E::Message>
    where
        C: Child<E::Message>,
        E: EventHandler + 'static,
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
        Self::Message: From<M>,
        M: 'static + Send;

    fn block<E, M, C>(self, Build<E>, C) -> Self::Walked
    where
        E: EventHandler<Message = M>,
        C: Child<M>,
        Self::Message: From<M>,
        E: 'static,
        M: 'static + Send;

    fn text(self, text: &str) -> Self::Walked;

    fn empty(self) -> Self::Walked;
}

pub trait Child<M>: 'static {
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

impl<M> Child<M> for String {
    fn walk<T>(self, walker: T) -> T::Walked where T: Walker, T::Message: From<M> {
        walker.text(&self)
    }
}

macro_rules! impl_child_num {
    ($($T:ty)*) => {
        $(
            impl<M> Child<M> for $T {
                fn walk<T>(self, walker: T) -> T::Walked
                where
                    T: Walker,
                    T::Message: From<M>,
                {
                    walker.text(&self.to_string())
                }
            }
        )*
    }
}

impl_child_num!(u8 u16 u32 u64 usize i8 i16 i32 i64 isize);

// TODO: Remove $Reference. Can be joined with $T.
macro_rules! impl_child_tuple {
    ($(($Reference:ident $($T:ident $idx:tt),*)),*,) => {$(
        impl<_M, $Reference, $($T),*> Child<_M> for ($Reference, $($T),*)
        where
            Self: Group<_M>,
            _M: 'static + Send,
            $Reference: 'static,
            $($T: 'static),*
        {
            fn walk<_T>(self, walker: _T) -> _T::Walked where _T: Walker, _T::Message: From<_M> {
                walker.group(self)
            }
        }

        impl<_M, $Reference, $($T),*> Group<_M> for ($Reference, $($T),*)
        where
            _M: 'static + Send,
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

// TODO: Rename to Consolidate
pub trait Consolidator {
    type Message;

    fn child<M, C>(&mut self, C) where C: Child<M>, Self::Message: From<M>, M: 'static;
}

pub trait Group<M> {
    fn consolidate<C>(self, C)
    where
        C: Consolidator,
        C::Message: From<M>;
}

pub trait Block: 'static {
    type Message: 'static + Send;
    type EventHandler: 'static + EventHandler<Message = Self::Message>;
    type Child: Child<Self::Message>;

    fn extract(self) -> BlockData<Self::EventHandler, Self::Child>;
}

pub struct BlockData<E, C> {
    pub data: Build<E>,
    pub child: C,
}

impl<E, C> Block for BlockData<E, C>
where
    E: 'static + EventHandler,
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

impl<B, M> Child<M> for B
where
    B: Block,
    B::EventHandler: 'static,
    B::Message: 'static,
    M: From<B::Message>,
    M: 'static + Send,
{
    fn walk<T>(self, walker: T) -> T::Walked
    where
        T: Walker,
        T::Message: From<M>,
    {
        let BlockData { data, child } = self.extract();

        proxy::Walk::new(walker).block(data, child)
    }
}
