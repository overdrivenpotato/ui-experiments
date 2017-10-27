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
}

pub trait Consolidator {
    type Message;

    fn child<C>(&mut self, child: C) where C: Child<Message = Self::Message>;
}

pub trait Group {
    type Message;

    fn consolidate<C>(self, &mut C) where C: Consolidator<Message = Self::Message>;
}

impl Group for ! {
    type Message = !;

    fn consolidate<C>(self, _: &mut C) {
        unreachable!()
    }
}

pub enum Grain<E, G, C> {
    Empty,
    Text(&'static str),
    Group(G),
    Block(Data<E>, C),
}

pub trait Child {
    type Message;
    type Group: Group<Message = Self::Message>;
    type Child: Child<Message = Self::Message>;
    type EventHandler: EventHandler<Message = Self::Message> + 'static;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child>;
}

impl Child for ! {
    type Message = !;
    type Group = !;
    type Child = !;
    type EventHandler = !;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
        unreachable!()
    }
}

impl Child for () {
    type Message = !;
    type Group = !;
    type Child = !;
    type EventHandler = !;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
        Grain::Empty
    }
}

impl Child for &'static str {
    type Message = !;
    type Group = !;
    type Child = !;
    type EventHandler = !;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
        Grain::Text(self)
    }
}

impl<B> Child for B where B: Block {
    type Message = B::Message;
    type Group = (B::Child,); // Unused.
    type Child = B::Child;
    type EventHandler = B::EventHandler;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
        let Baked { data, child } = self.extract();

        Grain::Block(data, child)
    }
}

macro_rules! impl_child_tuple {
    ($(($Reference:ident $($T:ident $idx:tt),*)),*,) => {$(
        impl<$Reference, $($T),*> Child for ($Reference, $($T),*)
        where
            $Reference: Child,
            $Reference::Message: 'static,
            $($T: Child<Message = $Reference::Message>),*
        {
            type Message = $Reference::Message;
            type Group = ($Reference, $($T),*);
            type Child = $Reference;
            type EventHandler = DefaultEvents<$Reference::Message>;

            fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
                Grain::Group(self)
            }
        }

        impl <$Reference, $($T),*> Group for ($Reference, $($T),*)
        where
            $Reference: Child,
            $($T: Child<Message = $Reference::Message>),*
        {
            type Message = $Reference::Message;

            fn consolidate<_C>(self, consolidator: &mut _C) where _C: Consolidator<Message = Self::Message> {
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

pub struct Baked<E, C> {
    pub data: Data<E>,
    pub child: C,
}

/// Wrapper around `Baked` to allow for easy use of `impl Trait`.
pub trait Block {
    /// Convenience type for messaging.
    type Message;
    type EventHandler: EventHandler<Message = Self::Message> + 'static;
    type Child: Child<Message = Self::Message>;

    fn extract(self) -> Baked<Self::EventHandler, Self::Child>;
}

impl<E, C> Block for Baked<E, C> where E: EventHandler + 'static, C: Child<Message = E::Message> {
    type Message = E::Message;
    type EventHandler = E;
    type Child = C;

    fn extract(self) -> Baked<Self::EventHandler, Self::Child> {
        self
    }
}

pub fn block<E, C>(data: Data<E>, child: C) -> impl Block<Message = E::Message>
where
    E: EventHandler + 'static,
    C: Child<Message = E::Message>,
{
    Baked { data, child }
}
