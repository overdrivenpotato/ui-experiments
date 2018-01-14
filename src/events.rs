use std::marker::PhantomData;
use std::ops::Sub;

pub enum Event {
    Render,
    Click(Coordinates),
    MouseDown(Coordinates, Button),
    MouseUp(Coordinates, Button),
}

pub struct Upgrade<E, M> {
    handler: E,
    _message: PhantomData<M>,
}

impl<E, M> EventHandler for Upgrade<E, M>
where
    E: EventHandler,
    M: 'static + Send + From<E::Message>,
{
    type Message = M;

    fn event(&self, event: Event) -> Option<Self::Message> {
        self.handler.event(event).map(M::from)
    }
}

impl<E, M> Upgrade<E, M>
where
    E: EventHandler,
    M: From<E::Message>
{
    pub fn new(handler: E) -> Self {
        Upgrade {
            handler,
            _message: PhantomData,
        }
    }
}

pub trait EventHandler: Send {
    type Message: 'static + Send;

    fn event(&self, event: Event) -> Option<Self::Message>;
}

impl<M, R, C, D, U> EventHandler for Events<M, R, C, D, U>
where
    R: Send + Fn() -> M,
    C: Send + Fn(Coordinates) -> M,
    D: Send + Fn(Coordinates, Button) -> M,
    U: Send + Fn(Coordinates, Button) -> M,
    M: 'static + Send,
{
    type Message = M;

    fn event(&self, event: Event) -> Option<Self::Message> {
        match event {
            Event::Render => self.render.as_ref().map(|r| r()),
            Event::Click(coordinates) => self.click.as_ref().map(|h| h(coordinates)),
            Event::MouseDown(coordinates, button) => self.down.as_ref().map(|h| h(coordinates, button)),
            Event::MouseUp(coordinates, button) => self.up.as_ref().map(|h| h(coordinates, button)),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Coordinates {
    pub x: u32,
    pub y: u32,
}

impl Into<Offset> for Coordinates {
    fn into(self) -> Offset {
        Offset {
            x: self.x as i32,
            y: self.y as i32,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Offset {
    pub x: i32,
    pub y: i32,
}

impl Sub<Offset> for Coordinates {
    type Output = Coordinates;

    fn sub(self, rhs: Offset) -> Self::Output {
        Coordinates {
            x: (self.x as i32 - rhs.x) as u32,
            y: (self.y as i32 - rhs.y) as u32,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Button {
    Left,
    Right,
    Middle,
}

pub struct Events<M, R, C, D, U>
where
    R: Fn() -> M,
    C: Fn(Coordinates) -> M,
    D: Fn(Coordinates, Button) -> M,
    U: Fn(Coordinates, Button) -> M
{
    render: Option<R>,
    click: Option<C>,
    down: Option<D>,
    up: Option<U>,
}

pub type DefaultEvents<M> = Events<
    M,
    fn() -> M,
    fn(Coordinates) -> M,
    fn(Coordinates, Button) -> M,
    fn(Coordinates, Button) -> M
>;

impl<M> DefaultEvents<M> {
    pub fn new() -> Self {
        Events {
            render: None,
            click: None,
            down: None,
            up: None,
        }
    }
}

impl<M, R, C, D, U> Events<M, R, C, D, U>
where
    R: Fn() -> M,
    C: Fn(Coordinates) -> M,
    D: Fn(Coordinates, Button) -> M,
    U: Fn(Coordinates, Button) -> M
{
    pub fn click<H>(self, handler: H) -> Events<M, R, H, D, U>
        where H: Fn(Coordinates) -> M
    {
        Events {
            render: self.render,
            click: Some(handler),
            down: self.down,
            up: self.up,
        }
    }

    pub fn mouse_down<H>(self, handler: H) -> Events<M, R, C, H, U>
        where H: Fn(Coordinates, Button) -> M
    {
        Events {
            render: self.render,
            click: self.click,
            down: Some(handler),
            up: self.up,
        }
    }

    pub fn mouse_up<H>(self, handler: H) -> Events<M, R, C, D, H>
        where H: Fn(Coordinates, Button) -> M
    {
        Events {
            render: self.render,
            click: self.click,
            down: self.down,
            up: Some(handler),
        }
    }

    pub fn render<H>(self, handler: H) -> Events<M, H, C, D, U>
    where
        H: Fn() -> M,
    {
        Events {
            render: Some(handler),
            click: self.click,
            down: self.down,
            up: self.up,
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
            .mouse_down(|_, _| 2)
            .mouse_up(|_, _| 3);

        let coordinates = Coordinates {
            x: 0,
            y: 0,
        };

        let button = Button::Left;

        assert_eq!(Some(1), events.event(Event::Click(coordinates)));
        assert_eq!(Some(2), events.event(Event::MouseDown(coordinates, button)));
        assert_eq!(Some(3), events.event(Event::MouseUp(coordinates, button)));
    }

    #[test]
    fn click() {
        let events = Events::new()
            .click(|Coordinates { x, y }| (x, y));

        assert_eq!(Some((1, 2)), events.event(Event::Click(Coordinates { x: 1, y: 2 })));
    }

    #[test]
    fn mouse_down() {
        let events = Events::new()
            .mouse_down(|Coordinates { x, y }, button| (x, y, button));

        assert_eq!(Some((1, 2, Button::Left)), events.event(Event::MouseDown(Coordinates { x: 1, y: 2 }, Button::Left)));
    }

    #[test]
    fn mouse_up() {
        let events = Events::new()
            .mouse_up(|Coordinates { x, y }, button| (x, y, button));

        assert_eq!(Some((1, 2, Button::Left)), events.event(Event::MouseUp(Coordinates { x: 1, y: 2 }, Button::Left)));
    }
}
