use stdweb::Value;
use stdweb::unstable::TryInto;
use stdweb::web::{Element, IEventTarget};
use stdweb::web::event::{self, IMouseEvent};

use events::{Coordinates, Event, EventHandler};

pub trait Bridge {
    fn bridge<E>(&mut self, event_handler: E) where E: EventHandler + 'static;
}

impl Bridge for Element {
    fn bridge<E>(&mut self, event_handler: E) where E: EventHandler + 'static {
        let element = self.clone();

        self.add_event_listener(move |click: event::ClickEvent| {
            let js_x = js! {
                function offset(element) {
                    var parentOffset = element.offsetParent ? offset(element.offsetParent) : 0;
                    return element.offsetLeft + parentOffset;
                }

                return offset(@{&element});
            };

            let js_y = js! {
                function offset(element) {
                    var parentOffset = element.offsetParent ? offset(element.offsetParent) : 0;
                    return element.offsetTop + parentOffset;
                }

                return offset(@{&element});
            };

            match (js_x, js_y) {
                (Value::Number(number_x), Value::Number(number_y)) => {
                    match (TryInto::<u32>::try_into(number_x), TryInto::<u32>::try_into(number_y)) {
                        (Ok(offset_x), Ok(offset_y)) => {
                            let x = click.client_x() as u32 - offset_x;
                            let y = click.client_y() as u32 - offset_y;

                            event_handler.event(Event::Click(Coordinates { x, y }));
                        }
                        _ => panic!("Failed to convert element offset to u32"),
                    }
                }
                // This should never happen.
                _ => unreachable!("Could not calculate element offsets"),
            }
        });

    }
}
