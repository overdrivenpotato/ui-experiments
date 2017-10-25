use std::marker::PhantomData;

use stdweb;
use stdweb::web::{self, INode, Element};

use blocks::{Block, Child, Data, Grain, Consolidator, Group};

use super::{App, State};

mod css;
mod events;

trait Bridge<T> {
    fn bridge(&mut self, T);
}

struct Processor<'a, N> where N: 'a {
    node: &'a mut N,
}

impl<'a, N> Processor<'a, N> where N: INode {
    fn new(node: &'a mut N) -> Self {
        Processor { node }
    }
}

impl<'a, N> Consolidator for Processor<'a, N> where N: INode {
    fn child<C>(&mut self, child: C) where C: Child {
        child.render(self.node);
    }
}

trait Render {
    fn render<N>(self, node: &mut N) where N: INode;
}

impl<C> Render for C where C: Child {
    fn render<N>(self, node: &mut N) where N: INode {
        match self.flatten() {
            Grain::Empty => {
                // NOOP
            },
            Grain::Text(text) => {
                let text = web::document().create_text_node(text);
                node.append_child(&text);
            },
            Grain::Group(group) => {
                let mut processor = Processor::new(node);
                group.consolidate(&mut processor);
            },
            Grain::Block(Data { style, event_handler }, child) => {
                let mut element = web::document().create_element("div");

                element.bridge(style);
                element.bridge(event_handler);

                node.append_child(&element);
                child.render(&mut element);
            },
        }
    }
}

struct Renderer<A, S, B> {
    app: A,
    root: Element,
    _state: PhantomData<S>,
    _block: PhantomData<B>,
}

impl<A, S, B> Renderer<A, S, B>
where
    A: App<S, B>,
    B: Block,
    S: State<Message = B::Message>,
{
    fn new(app: A, root: Element) -> Self {
        Renderer {
            app,
            root,
            _state: PhantomData,
            _block: PhantomData,
        }
    }

    pub fn render(&mut self, state: S) {
        self.app.render(state).render(&mut self.root);
    }
}

/// Launch the app with a root element ID.
pub fn launch<S, B, A>(root: &'static str, app: A)
where
    A: App<S, B>,
    B: Block,
    S: State<Message = B::Message>,
{
    stdweb::initialize();

    if let Some(root) = web::document().get_element_by_id(&root) {
        if let Some(mut parent) = root.parent_node() {
            let mut element = web::document().create_element("div");
            parent.replace_child(&element, &root);

            let mut renderer = Renderer::new(app, element.clone());

            renderer.render(Default::default());
        }
    } else {
        eprintln!("Could not find #{}", root);
    }

    stdweb::event_loop();
}
