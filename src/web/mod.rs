use stdweb::{self, Value};
use stdweb::unstable::TryInto;
use stdweb::web::{self, INode, IEventTarget};
use stdweb::web::event::IMouseEvent;

use blocks::{Block, Child, Data, Grain, Consolidator, Group};
use events::{Coordinates, Event, EventHandler};

mod css;

use self::css::Inline;

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
                let style = style.inline();

                js! { @{&element}.setAttribute("style", @{style}) }

                let element_clone = element.clone();
                element.add_event_listener(move |click: web::event::ClickEvent| {
                    let js_x = js! {
                        function offset(element) {
                            var parentOffset = element.offsetParent ? offset(element.offsetParent) : 0;
                            return element.offsetLeft + parentOffset;
                        }

                        return offset(@{&element_clone});
                    };

                    let js_y = js! {
                        function offset(element) {
                            var parentOffset = element.offsetParent ? offset(element.offsetParent) : 0;
                            return element.offsetTop + parentOffset;
                        }

                        return offset(@{&element_clone});
                    };

                    match (js_x, js_y) {
                        (Value::Number(number_x), Value::Number(number_y)) => {
                            match (TryInto::<u32>::try_into(number_x), TryInto::<u32>::try_into(number_y)) {
                                (Ok(offset_x), Ok(offset_y)) => {
                                    let x = click.client_x() as u32 - offset_x;
                                    let y = click.client_y() as u32 - offset_y;

                                    event_handler.event(Event::Click(Coordinates { x, y }));
                                }
                                _ => panic!("Failed to convert element offset to number"),
                            }
                        }
                        // This should never happen.
                        _ => unreachable!("Could not calculate element offsets"),
                    }
                });

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
