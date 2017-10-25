use stdweb;
use stdweb::web::INode;

use ui::Style;
use blocks::{Block, Child, Grain, Consolidator, Group};
use events::EventHandler;

pub struct Web<'a> {
    query: &'a str,
}

pub fn mount<'a>(query: &'a str) -> Web<'a> {
    Web { query }
}

struct Processor {
    rendered: Vec<String>,
}

impl Processor {
    fn new() -> Self {
        Processor {
            rendered: Vec::new(),
        }
    }

    fn render() -> String {
        self.rendered.join(" ")
    }
}

impl Consolidator for Processor {
    fn child<C>(&mut self, child: C) where C: Child {
        self.rendered.push(child.div());
    }
}

trait Div {
    fn div(self) -> String;
}

impl<C> Div for C where C: Child {
    fn div(self) -> String {
        match self.flatten() {
            Grain::Empty => String::new(),
            Grain::Text(t) => String::from(t),
            Grain::Group(group) => {
                let mut processor = Processor::new();
                group.consolidate(&mut processor);
                processor.render()
            },
            Grain::Block(_, child) => format!("<div>{}</div>", child.div()),
        }
    }
}

impl<'a> Web<'a> {
    pub fn launch<T>(self, element: T) where T: Block {
        stdweb::initialize();

        println!("Mounting on {}", self.query);
        println!("{}", element.div());

        stdweb::event_loop();
    }
}
