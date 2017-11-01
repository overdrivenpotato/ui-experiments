use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use stdweb;
use stdweb::web::{self, INode, Element};

use events::EventHandler;
use blocks::{Block, Child, Build, Consolidator, Group, Walker};

use super::{App, State};

mod css;
mod events;

use self::css::SetStyle;
use self::events::SetHandler;

struct Processor<'a, N, U> where N: 'a {
    node: &'a mut N,
    updater: U,
}

impl<'a, N, U> Processor<'a, N, U> {
    fn new(node: &'a mut N, updater: U) -> Self {
        Processor { node, updater }
    }
}

impl<'a, N, U> Consolidator for Processor<'a, N, U>
where
    U: Messenger + 'static,
    N: INode,
{
    type Message = U::Message;

    fn child<M, C>(&mut self, child: C) where C: Child<M>, Self::Message: From<M>, M: 'static {
        let processor = Processor::new(self.node, MessageConverter::wrap(self.updater.clone()));
        child.walk(processor);
    }
}

impl<'a, N, U> Walker for Processor<'a, N, U>
where
    N: INode,
    U: Messenger + 'static,
    U::Message: 'static,
{
    type Message = U::Message;
    type Walked = Self;

    fn group<M, G>(self, group: G) -> Self
    where
        G: Group<M>,
        Self::Message: From<M>,
    {
        {
            let processor = Processor::new(self.node, self.updater.clone());
            group.consolidate(processor);
        }

        self
    }

    fn block<E, M, C>(self, Build { style, event_handler }: Build<E>, child: C) -> Self::Walked
    where
        E: EventHandler<Message = M>,
        C: Child<M>,
        Self::Message: From<M>,
        E: 'static,
        M: 'static,
    {
        let mut element = web::document().create_element("div");

        element.set_style(style);
        element.set_handler(event_handler, MessageConverter::wrap(self.updater.clone()));

        self.node.append_child(&element);
        child.walk(Processor::new(&mut element, MessageConverter::wrap(self.updater.clone())));

        self
    }

    fn text(self, text: &'static str) -> Self {
        let text = web::document().create_text_node(text);
        self.node.append_child(&text);

        self
    }

    fn empty(self) -> Self {
        self
    }
}

struct MessageConverter<M, A> {
    messenger: M,
    _a: PhantomData<A>,
}

impl<M, A> MessageConverter<M, A> {
    fn wrap(messenger: M) -> Self {
        MessageConverter { messenger, _a: PhantomData }
    }
}

impl<M, A> Clone for MessageConverter<M, A> where M: Clone {
    fn clone(&self) -> Self {
        MessageConverter {
            messenger: self.messenger.clone(),
            _a: PhantomData,
        }
    }
}

impl<A, B, M> Messenger for MessageConverter<M, A>
where
    B: From<A>,
    M: Messenger<Message = B>,
{
    type Message = A;

    fn message(&self, message: Self::Message) {
        self.messenger.message(message.into());
    }
}

pub trait Messenger: Clone {
    type Message;

    fn message(&self, message: Self::Message);
}

struct Renderer<A, S, B> {
    app: A,
    root: Element,
    state: S,
    _block: PhantomData<B>,
}

impl<A, S, B> Renderer<A, S, B>
where
    A: App<S, B> + 'static,
    S: State + 'static,
    B: Block<Message = S::Message> + 'static,
    B::Child: Child<S::Message>,
{
    fn start(app: A, root: Element, state: S) {
        let renderer: Renderer<A, S, B> = Renderer {
            app,
            root,
            state,
            _block: PhantomData,
        };

        let updater = Rc::new(RefCell::new(renderer));

        updater.update();
    }
}

/// Helper trait for updates.
trait Update {
    fn update(&self);
}

impl<A, S, B> Update for Rc<RefCell<Renderer<A, S, B>>>
where
    A: App<S, B> + 'static,
    S: State + 'static,
    B: Block<Message = S::Message> + 'static,
    B::Child: Child<S::Message>,
{
    fn update(&self) {
        let mut guard = self.borrow_mut();

        // Remove the old render.
        {
            let element = guard.root.as_node();

            js! {
                var node = @{element};

                while (node.hasChildNodes()) {
                    node.removeChild(node.lastChild);
                }
            }
        }

        // Recreate the entire DOM structure.
        let _ = guard.app
            .render(&guard.state)
            .walk(Processor::new(&mut guard.root, self.clone()));
    }
}

impl<A, S, B> Messenger for Rc<RefCell<Renderer<A, S, B>>>
where
    A: App<S, B> + 'static,
    B: Block + 'static,
    S: State<Message = B::Message> + 'static,
{
    type Message = B::Message;

    fn message(&self, message: Self::Message) {
        let mut guard = self.borrow_mut();

        // Reduce the state without generating a default stub state.
        let old_state = mem::replace(&mut guard.state, unsafe { mem::uninitialized() });
        guard.state = old_state.reduce(message);

        drop(guard);

        self.update();
    }
}

/// Launch the app with a root element ID.
pub fn launch<S, B, A>(root: &'static str, app: A)
where
    A: App<S, B> + 'static,
    B: Block + 'static,
    S: State<Message = B::Message> + 'static,
{
    stdweb::initialize();

    if let Some(root) = web::document().get_element_by_id(&root) {
        if let Some(mut parent) = root.parent_node() {
            let mut element = web::document().create_element("div");
            parent.replace_child(&element, &root);

            Renderer::start(
                app,
                element.clone(),
                Default::default(),
            );
        }
    } else {
        eprintln!("Could not find #{}", root);
    }

    stdweb::event_loop();
}
