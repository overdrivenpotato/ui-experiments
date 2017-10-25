pub enum Event {
    Click(Coordinates),
    Down(Coordinates, Button),
    Up(Coordinates, Button),
}

pub trait EventHandler {
    type Message;

    fn event(&self, event: Event) -> Option<Self::Message>;
}

impl EventHandler for ! {
    type Message = !;

    fn event(&self, event: Event) -> Option<Self::Message> {
        unreachable!()
    }
}

impl<M, C, D, U> EventHandler for Events<M, C, D, U>
    where C: Fn(Coordinates) -> M,
          D: Fn(Coordinates, Button) -> M,
          U: Fn(Coordinates, Button) -> M
{
    type Message = M;

    fn event(&self, event: Event) -> Option<Self::Message> {
        match event {
            Event::Click(coordinates) => self.click.as_ref().map(|h| h(coordinates)),
            Event::Down(coordinates, button) => self.down.as_ref().map(|h| h(coordinates, button)),
            Event::Up(coordinates, button) => self.up.as_ref().map(|h| h(coordinates, button)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Button {
    Left,
    Right,
    Middle,
}

pub struct Events<M, C, D, U>
    where C: Fn(Coordinates) -> M,
          D: Fn(Coordinates, Button) -> M,
          U: Fn(Coordinates, Button) -> M
{
    click: Option<C>,
    down: Option<D>,
    up: Option<U>,
}

type DefaultEvents<M> = Events<
    M,
    fn(Coordinates) -> M,
    fn(Coordinates, Button) -> M,
    fn(Coordinates, Button) -> M
>;

pub type EmptyEvents = DefaultEvents<!>;

impl<M> DefaultEvents<M> {
    pub fn new() -> Self {
        Events {
            click: None,
            down: None,
            up: None,
        }
    }
}

impl<M, C, D, U> Events<M, C, D, U>
    where C: Fn(Coordinates) -> M,
          D: Fn(Coordinates, Button) -> M,
          U: Fn(Coordinates, Button) -> M
{
    pub fn click<H>(self, handler: H) -> Events<M, H, D, U>
        where H: Fn(Coordinates) -> M
    {
        Events {
            click: Some(handler),
            down: self.down,
            up: self.up,
        }
    }

    pub fn down<H>(self, handler: H) -> Events<M, C, H, U>
        where H: Fn(Coordinates, Button) -> M
    {
        Events {
            click: self.click,
            down: Some(handler),
            up: self.up,
        }
    }

    pub fn up<H>(self, handler: H) -> Events<M, C, D, H>
        where H: Fn(Coordinates, Button) -> M
    {
        Events {
            click: self.click,
            down: self.down,
            up: Some(handler),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn messages() {
        let events = Events::new()
            .click(|_| 1)
            .down(|_, _| 2)
            .up(|_, _| 3);

        let coordinates = Coordinates {
            x: 0,
            y: 0,
        };

        let button = Button::Left;

        assert_eq!(Some(1), events.event(Event::Click(coordinates)));
        assert_eq!(Some(2), events.event(Event::Down(coordinates, button)));
        assert_eq!(Some(3), events.event(Event::Up(coordinates, button)));
    }

    #[test]
    fn click() {
        let events = Events::new()
            .click(|Coordinates { x, y }| (x, y));

        assert_eq!(Some((1, 2)), events.event(Event::Click(Coordinates { x: 1, y: 2 })));
    }

    #[test]
    fn down() {
        let events = Events::new()
            .down(|Coordinates { x, y }, button| (x, y, button));

        assert_eq!(Some((1, 2, Button::Left)), events.event(Event::Down(Coordinates { x: 1, y: 2 }, Button::Left)));
    }

    #[test]
    fn up() {
        let events = Events::new()
            .up(|Coordinates { x, y }, button| (x, y, button));

        assert_eq!(Some((1, 2, Button::Left)), events.event(Event::Up(Coordinates { x: 1, y: 2 }, Button::Left)));
    }
}
