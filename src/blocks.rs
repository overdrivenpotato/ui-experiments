use ui::Style;

use events::{DefaultEvents, EmptyEvents, EventHandler};

pub struct Data<E> {
    pub style: Style,
    pub event_handler: E,
}

impl Default for Data<EmptyEvents> {
    fn default() -> Self {
        Data {
            style: Default::default(),
            event_handler: EmptyEvents::new(),
        }
    }
}

impl<T> Data<DefaultEvents<T>> {
    fn passthrough() -> Self {
        Data {
            style: Default::default(),
            event_handler: DefaultEvents::new(),
        }
    }
}

impl<E> Data<E> {
    pub fn with(style: Style, event_handler: E) -> Data<E> {
        Data { style, event_handler }
    }

    pub fn style(self, style: Style) -> Data<E> {
        Data { style, ..self }
    }

    pub fn events<H>(self, handler: H) -> Data<H> {
        Data {
            style: self.style,
            event_handler: handler,
        }
    }

    pub fn block<C>(self, child: C) -> impl Block<Message = E::Message>
    where
        C: Child,
        E: EventHandler + 'static,
        E::Message: From<C::Message>,
    {
        BlockData { data: self, child }
    }
}

pub trait Consolidator {
    type Message;

    fn child<C>(&mut self, C)
    where
        C: Child,
        Self::Message: From<C::Message>,
        C::Message: 'static;
}

pub trait Group {
    type Message;

    fn consolidate<C>(self, C) where C: Consolidator<Message = Self::Message>;
}

pub trait Walker {
    type Message;

    fn group<G>(self, G) -> Self where G: Group<Message = Self::Message>;

    fn block<E, C>(self, Data<E>, C) -> Self
    where
        E: EventHandler<Message = Self::Message> + 'static,
        C: Child,
        Self::Message: From<C::Message>;

    fn text(self, text: &'static str) -> Self;
}

pub trait Child {
    type Message: 'static;

    fn walk<T>(self, T) -> T where T: Walker<Message = Self::Message>;
}

impl Child for &'static str {
    type Message = !;

    fn walk<T>(self, walker: T) -> T where T: Walker<Message = Self::Message> {
        walker.text(self)
    }
}

impl Child for () {
    type Message = !;

    fn walk<T>(self, walker: T) -> T where T: Walker<Message = Self::Message> {
        walker
    }
}

impl<B> Child for B where B: Block {
    type Message = B::Message;

    fn walk<T>(self, walker: T) -> T where T: Walker<Message = Self::Message> {
        let BlockData { data, child } = self.extract();

        walker.block(data, child)
    }
}

macro_rules! impl_child_tuple {
    ($(($Reference:ident $($T:ident $idx:tt),*)),*,) => {$(
        impl<$Reference, $($T),*> Child for ($Reference, $($T),*)
        where
            $Reference: Child,
            $($T: Child<Message = $Reference::Message>),*
        {
            type Message = $Reference::Message;

            fn walk<_T>(self, walker: _T) -> _T where _T: Walker<Message = Self::Message> {
                walker.group(self)
            }
        }

        impl <$Reference, $($T),*> Group for ($Reference, $($T),*)
        where
            $Reference: Child,
            $($T: Child<Message = $Reference::Message>),*
        {
            type Message = $Reference::Message;

            fn consolidate<_C>(self, mut consolidator: _C) where _C: Consolidator<Message = Self::Message> {
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

pub struct BlockData<E, C> {
    pub data: Data<E>,
    pub child: C,
}

/// Wrapper around `BlockData` to allow for easy use of `impl Trait`.
pub trait Block {
    /// Convenience type for messaging.
    type Message: From<<Self::Child as Child>::Message> + 'static;
    type EventHandler: EventHandler<Message = Self::Message> + 'static;
    type Child: Child;

    fn extract(self) -> BlockData<Self::EventHandler, Self::Child>;
}

impl<E, C> Block for BlockData<E, C>
where
    E: EventHandler + 'static,
    C: Child,
    E::Message: From<C::Message>,
{
    type Message = E::Message;
    type EventHandler = E;
    type Child = C;

    fn extract(self) -> BlockData<Self::EventHandler, Self::Child> {
        self
    }
}

pub fn block<M, C>(child: C) -> impl Block<Message = M>
where
    C: Child,
    M: From<C::Message> + 'static,
{
    BlockData { data: Data::passthrough(), child }
}
