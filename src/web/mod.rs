use std::marker::PhantomData;
use std::rc::Rc;
use std::cell::RefCell;
use std::mem;

use stdweb;
use stdweb::web::{self, INode, Element};

use events::EventHandler;
use blocks::{Block, Child, Data, Grain, Consolidator, Group};

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
    N: INode,
    U: Messenger + 'static,
{
    type Message = U::Message;

    fn child<C>(&mut self, child: C)
    where
        C: Child<Message = Self::Message>,
    {
        child.flatten().insert(self.node, self.updater.clone());
    }
}

trait Insert {
    type Message;

    fn insert<N, U>(self, node: &mut N, updater: U)
    where
        N: INode,
        U: Messenger<Message = Self::Message> + 'static;
}

impl<E, G, C> Insert for Grain<E, G, C>
where
    E: EventHandler + 'static,
    G: Group<Message = E::Message>,
    C: Child<Message = E::Message>,
{
    type Message = E::Message;

    fn insert<N, U>(self, node: &mut N, updater: U)
    where
        N: INode,
        U: Messenger<Message = Self::Message> + 'static,
    {
        match self {
            Grain::Empty => {
                // NOOP
            },
            Grain::Text(text) => {
                let text = web::document().create_text_node(text);
                node.append_child(&text);
            },
            Grain::Group(group) => {
                let mut processor = Processor::new(node, updater);
                group.consolidate(&mut processor);
            },
            Grain::Block(Data { style, event_handler }, child) => {
                let mut element = web::document().create_element("div");

                element.set_style(style);
                element.set_handler(event_handler, updater.clone());

                node.append_child(&element);
                child.flatten().insert(&mut element, updater);
            },
        }
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

trait Update {
    fn update(&self);
}

impl<A, S, B> Update for Rc<RefCell<Renderer<A, S, B>>>
where
    A: App<S, B> + 'static,
    S: State + 'static,
    B: Block<Message = S::Message> + 'static,
{
    fn update(&self) {
        let mut guard = self.borrow_mut();

        let rendered = guard.app.render(&guard.state).flatten();

        rendered.insert(&mut guard.root, self.clone());
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
