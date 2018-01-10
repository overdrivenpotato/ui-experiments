use std::mem;
use std::marker::PhantomData;
use std::sync::{Arc, Mutex};
use std::collections::HashSet;

use ::{State, Reactor};
use ui::Style;
use block::{proxy, Block, BlockData, Walker, Group, Child, Build, Consolidator};
use events::{EventHandler, Upgrade};

use self::css::Inline;
use self::ffi::{Attribute, EventType};
use self::atom::Atom;

#[doc(hidden)]
pub mod ffi;
mod css;
mod atom;

enum Rendered {
    Text(String),
    Element {
        children: Vec<Atom>,
        attributes: Vec<Attribute>,
        registered_events: HashSet<EventType>,
    },
}

pub enum Candidate<M> {
    Text(String),
    Element {
        children: Vec<Candidate<M>>,
        attributes: Vec<Attribute>,
        event_handler: Box<EventHandler<Message = M>>,
    },
}

impl<B> From<B> for Candidate<B::Message>
where
    B: Block,
    B::EventHandler: 'static,
    B::Message: 'static,
{
    fn from(block: B) -> Self {
        let BlockData { child, data } = block.extract();

        Candidate::Element {
            attributes: vec![
                Attribute::new("style", data.style.inline())
            ],
            event_handler: Box::new(data.event_handler),
            children: child.walk(BakedWalker::<B::Message>::new()).to_candidate(),
        }
    }
}

enum BakedChild<M> {
    Text(String),
    Empty,
    Group(Vec<BakedChild<M>>),
    Element {
        style: Style,
        events: Box<EventHandler<Message = M>>,
        child: Box<BakedChild<M>>,
    },
}

impl<M> BakedChild<M> {
    fn to_candidate(self) -> Vec<Candidate<M>> {
        match self {
            BakedChild::Text(t) => vec![Candidate::Text(t)],
            BakedChild::Empty => vec![],
            BakedChild::Group(children) =>
                children
                    .into_iter()
                    .flat_map(BakedChild::to_candidate)
                    .collect(),
            BakedChild::Element { child, style, events } => vec![
                Candidate::Element {
                    attributes: vec![
                        Attribute::new("style", style.inline()),
                    ],
                    event_handler: events,
                    children: child.to_candidate(),
                },
            ],
        }
    }
}

struct BakedWalker<M> {
    _message: PhantomData<M>,
}

impl<M> BakedWalker<M> {
    fn new() -> Self {
        BakedWalker {
            _message: PhantomData
        }
    }
}

impl<M> Walker for BakedWalker<M> where M: 'static + Send {
    type Message = M;
    type Walked = BakedChild<M>;

    fn group<M_, G>(self, group: G) -> Self::Walked
    where
        G: Group<M_>,
        Self::Message: From<M_>,
        M_: 'static + Send,
    {
        struct ConsolidatorImpl<M> {
            _message: PhantomData<M>,
            children: Vec<BakedChild<M>>,
        }

        impl<'a, M> Consolidator for &'a mut ConsolidatorImpl<M> where M: 'static + Send {
            type Message = M;

            fn child<M_, C>(&mut self, child: C)
            where
                C: Child<M_>,
                Self::Message: From<M_>,
                M_: 'static
            {
                self.children.push(child.walk(BakedWalker::<M>::new()));
            }
        }

        let mut consolidator = ConsolidatorImpl::<M> {
            children: Vec::new(),
            _message: PhantomData,
        };

        {
            let consolidator = proxy::Consolidate::new(&mut consolidator);

            group.consolidate(consolidator);
        }

        BakedChild::Group(consolidator.children)
    }

    fn block<E, M_, C>(self, data: Build<E>, child: C) -> Self::Walked
    where
        E: EventHandler<Message = M_>,
        C: Child<M_>,
        Self::Message: From<M_>,
        E: 'static,
        M_: 'static + Send,
    {
        BakedChild::Element {
            style: data.style,
            events: Box::new(Upgrade::new(data.event_handler)),
            child: Box::new(child.walk(BakedWalker::<M>::new())),
        }
    }

    fn text(self, text: &str) -> Self::Walked {
        BakedChild::Text(text.to_string())
    }

    fn empty(self) -> Self::Walked {
        BakedChild::Empty
    }
}

pub struct Instance<S, F, B>
where
    F: Fn(&S) -> B
{
    root: Atom,
    state: S,
    app: F,
}

impl<S, F, B> Instance<S, F, B> where F: Fn(&S) -> B, S: State {
    fn wrap(root: Atom, state: S, app: F) -> Self {
        Self { root, state, app }
    }

    fn render(&self) -> B {
        (self.app)(&self.state)
    }

    fn root(&mut self) -> &mut Atom {
        &mut self.root
    }

    fn reduce(&mut self, message: S::Message) {
        self.state.reduce(message);
    }
}

pub struct Handle<S, F, B> where F: Fn(&S) -> B {
    instance: Arc<Mutex<Instance<S, F, B>>>,
}

impl<S, F, B> Handle<S, F, B>
where
    B: Block,
    S: State<Message = B::Message>,
    F: 'static + Send + Fn(&S) -> B,
{
    /// Re render the app.
    fn render(&self) {
        let mut guard = self.instance.lock().unwrap();
        let candidate = Candidate::from(guard.render());
        guard.root().upgrade(candidate, self.clone());
    }

    fn message(&self, message: S::Message) {
        let mut instance = self.instance.lock().unwrap();
        instance.reduce(message);
    }
}

impl<S, F, B> Clone for Handle<S, F, B>
where
    F: Fn(&S) -> B
{
    fn clone(&self) -> Self {
        Self { instance: self.instance.clone() }
    }
}

impl<S, F, B> From<Instance<S, F, B>> for Handle<S, F, B>
where
    F: Fn(&S) -> B,
{
    fn from(instance: Instance<S, F, B>) -> Self {
        Self { instance: Arc::new(Mutex::new(instance)) }
    }
}

pub trait Update: Clone + Send + 'static {
    type Message: 'static + Send;

    fn reduce(&self, message: Self::Message);
}

impl<S, F, B> Update for Handle<S, F, B>
where
    Self: 'static,
    B: Block,
    S: State<Message = B::Message>,
    F: Send + Fn(&S) -> B,
{
    type Message = S::Message;

    fn reduce(&self, message: Self::Message) {
        self.message(message);
        self.render();
    }
}

/// The internal entry point to the app.
///
/// This function is deferred until the wasm module has been loaded correctly.
/// Because of this, the callee must preserve arguments and create a
/// `ffi::defer` hook in order to allow use of module imports.
fn internal_launch<F, B, S>(app: F)
where
    B: Block,
    B::Message: 'static,
    B::EventHandler: 'static,
    F: 'static + Send + Fn(&S) -> B,
    S: State<Message = B::Message>,
{
    css::inject();

    let root = Atom::mount();
    let state = S::new(Reactor::new());
    let instance = Instance::wrap(root, state, app);

    let handle: Handle<S, F, B> = instance.into();

    handle.render();

    // Ensure that the app cannot be destroyed.
    mem::forget(handle);
}

/// Launch an app.
pub fn launch<F, B, S>(app: F)
where
    B: Block,
    B::Message: 'static,
    B::EventHandler: 'static,
    F: 'static + Send + Fn(&S) -> B,
    S: State<Message = B::Message>,
{
    static mut APP: Option<Box<Box<Fn()>>> = None;
    static mut TRIGGER: Option<Box<Fn()>> = None;

    unsafe {
        TRIGGER = Some(Box::new(|| {
            let app: Option<Box<Box<F>>> = mem::replace(mem::transmute(&mut APP), None);

            match app {
                Some(app) => internal_launch(**app),
                _ => unreachable!(),
            }
        }));

        APP = Some(mem::transmute(Box::new(Box::new(app))));
    }

    extern fn trigger() {
        unsafe {
            match mem::replace(&mut TRIGGER, None) {
                Some(f) => f(),
                _ => unreachable!(),
            }
        }
    }

    ffi::defer(trigger);
}

/// Launch an app with no state. Useful for static views and prototyping.
pub fn simple<F, B>(app: F)
where
    B: Block,
    B::Message: 'static,
    B::EventHandler: 'static,
    F: 'static + Send + Fn() -> B,
{
    struct EmptyState<T> { _message: PhantomData<T> }
    impl<T> State for EmptyState<T> where T: 'static + Send {
        type Message = T;

        fn new(_reactor: Reactor<Self::Message>) -> Self {
            EmptyState {
                _message: PhantomData
            }
        }

        fn reduce(&mut self, _message: Self::Message) {}
    }

    launch(move |_: &EmptyState<B::Message>| app())
}
