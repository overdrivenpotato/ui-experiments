use stdweb::Value;
use stdweb::unstable::TryInto;
use stdweb::web::{Element, IEventTarget};
use stdweb::web::event::{self, IMouseEvent};

use web::Bridge;
use events::{Coordinates, Event, EventHandler};

trait PageCoordinates {
    fn page_coordinates(&self) -> Coordinates;
}

impl PageCoordinates for Element {
    fn page_coordinates(&self) -> Coordinates {
        let js_x = js! {
            function offset(element) {
                var parentOffset = element.offsetParent ? offset(element.offsetParent) : 0;
                return element.offsetLeft + parentOffset;
            }

            return offset(@{&*self});
        };

        let js_y = js! {
            function offset(element) {
                var parentOffset = element.offsetParent ? offset(element.offsetParent) : 0;
                return element.offsetTop + parentOffset;
            }

            return offset(@{&*self});
        };

        match (js_x, js_y) {
            (Value::Number(number_x), Value::Number(number_y)) => {
                match (TryInto::<u32>::try_into(number_x), TryInto::<u32>::try_into(number_y)) {
                    (Ok(x), Ok(y)) => Coordinates { x, y },
                    _ => panic!("Failed to convert element offset to u32"),
                }
            }
            // This should never happen.
            _ => unreachable!("Could not calculate element offsets"),
        }
    }
}

impl<E> Bridge<E> for Element where E: EventHandler + 'static {
    fn bridge(&mut self, event_handler: E) {
        let element = self.clone();

        self.add_event_listener(move |click: event::ClickEvent| {
            let client = Coordinates {
                x: click.client_x() as u32,
                y: click.client_y() as u32,
            };

            event_handler.event(Event::Click(client - element.page_coordinates().into()));
        });
    }
}
