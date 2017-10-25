use stdweb;
use stdweb::web::{self, INode};

use blocks::{Block, Child, Grain, Consolidator, Group};

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
            Grain::Block(_, child) => {
                let mut element = web::document().create_element("div");
                node.append_child(&element);
                child.render(&mut element);
            },
        }
    }
}

/// Launch the app with a root element ID.
pub fn launch<T>(root: &'static str, block: T) where T: Block {
    stdweb::initialize();

    if let Some(mut root) = web::document().get_element_by_id(&root) {
        block.render(&mut root);
    } else {
        eprintln!("Could not find #{}", root);
    }

    stdweb::event_loop();
}
