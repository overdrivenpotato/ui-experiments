use ui::{font, Color, Style};
use ui::position::Position;

use events::{EmptyEvents, Coordinates, Button, Events, EventHandler};

pub struct Data<E> {
    style: Style,
    event_handler: E,
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
    fn with(style: Style, event_handler: E) -> Data<E> {
        Data { style, event_handler }
    }

    fn style(self, style: Style) -> Data<E> {
        Data { style, ..self }
    }

    fn events<H>(self, handler: H) -> Data<H> {
        Data {
            style: self.style,
            event_handler: handler,
        }
    }
}

pub trait Consolidator {
    fn child<C>(&mut self, child: C) where C: Child;
}

pub trait Group {
    fn consolidate<C>(self, &mut C) where C: Consolidator;
}

impl Group for ! {
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
    type Group: Group;
    type Child: Child;
    type EventHandler: EventHandler;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child>;
}

impl Child for ! {
    type Group = !;
    type Child = !;
    type EventHandler = !;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
        unreachable!()
    }
}

impl Child for () {
    type Group = !;
    type Child = !;
    type EventHandler = !;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
        Grain::Empty
    }
}

impl Child for &'static str {
    type Group = !;
    type Child = !;
    type EventHandler = !;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
        Grain::Text(self)
    }
}

impl<B> Child for B
where
    B: Block,
    B::Child: Child,
    B::EventHandler: EventHandler,
{
    type Group = !;
    type Child = B::Child;
    type EventHandler = B::EventHandler;

    fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
        let Baked { data, child } = self.extract();

        Grain::Block(data, child)
    }
}

macro_rules! impl_child_tuple {
    ($(($($T:ident $idx:tt),*)),*,) => {$(
        impl<$($T),*> Child for ($($T),*,)
        where
            $($T: Child),*
        {
            type Group = ($($T),*,);
            type Child = !;
            type EventHandler = !;

            fn flatten(self) -> Grain<Self::EventHandler, Self::Group, Self::Child> {
                Grain::Group(self)
            }
        }

        impl <$($T),*> Group for ($($T),*,)
        where
            $($T: Child),*
        {
            fn consolidate<_C>(self, consolidator: &mut _C) where _C: Consolidator {
                $(consolidator.child(self.$idx);)*
            }
        }
    )*}
}

impl_child_tuple! {
    (A 0),
    (A 0, B 1),
    (A 0, B 1, C 2),
    (A 0, B 1, C 2, D 3),
    (A 0, B 1, C 2, D 3, E 4),
    (A 0, B 1, C 2, D 3, E 4, F 5),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24),
    (A 0, B 1, C 2, D 3, E 4, F 5, G 6, H 7, I 8, J 9, K 10, L 11, M 12, N 13, O 14, P 15, Q 16, R 17, S 18, T 19, U 20, V 21, W 22, X 23, Y 24, Z 25),
}

pub struct Baked<E, C> {
    pub data: Data<E>,
    pub child: C,
}

/// Wrapper around `Baked` to allow for easy use of `impl Trait`.
pub trait Block {
    type Message;
    type EventHandler: EventHandler<Message = Self::Message>;
    type Child: Child;

    fn extract(self) -> Baked<Self::EventHandler, Self::Child>;
}

impl<E, C> Block for Baked<E, C> where E: EventHandler, C: Child {
    type Message = E::Message;
    type EventHandler = E;
    type Child = C;

    fn extract(self) -> Baked<Self::EventHandler, Self::Child> {
        self
    }
}

fn block<E, C>(data: Data<E>, child: C) -> impl Block<Message = E::Message>
where
    E: EventHandler,
    C: Child,
{
    Baked { data, child }
}

pub fn test() -> impl Block<Message = ()> {
    let style = Style {
        position: Position::Anchor,
        font: font::Font {
            family: String::from("Arial"),
            weight: font::Weight::Regular,
            style: font::Style::Italic,
            color: Color::black(),
        },
        .. Style::default()
    };

    let events = Events::new()
        .click(|Coordinates { x, y }| println!("Mouse clicked at {}, {}", x, y))
        .up(|Coordinates { x, y }, button| {
            match button {
                Button::Left => println!("Left mouse up at {}, {}", x, y),
                _ => println!("Mouse up with another button at {}, {}", x, y),
            }
        });

    block(Data::with(style, events), (
        block(Data::default(), (
            "Sub",
            "Test",
        )),
        "Testing",
        "123",
        "456",
    ))
}
