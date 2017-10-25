use std::rc::Rc;

use stdweb::Value;
use stdweb::unstable::TryInto;
use stdweb::web::{Element, IEventTarget};
use stdweb::web::event::{self, IMouseEvent};

use web::Bridge;
use events::{Button, Coordinates, Event, EventHandler};

trait FromMouseCode {
    fn from_mouse_code(self) -> Button;
}

impl FromMouseCode for u32 {
    fn from_mouse_code(self) -> Button {
        match self {
            0 => Button::Left,
            1 => Button::Middle,
            2 => Button::Right,
            _ => Button::Left,
        }
    }
}

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
        let event_handler = Rc::new(event_handler);

        let page_coordinates = self.page_coordinates();

        let event_handler_click = event_handler.clone();
        self.add_event_listener(move |click: event::ClickEvent| {
            let client = Coordinates {
                x: click.client_x() as u32,
                y: click.client_y() as u32,
            };

            event_handler_click.event(Event::Click(client - page_coordinates.into()));
        });

        let event_handler_mouse_down = event_handler.clone();
        let mouse_down = move |x: u32, y: u32, button: u32| {
            let event = Event::Down(Coordinates { x, y }, button.from_mouse_code());
            event_handler_mouse_down.event(event);
        };

        let event_handler_mouse_up = event_handler.clone();
        let mouse_up = move |x: u32, y: u32, button: u32| {
            let event = Event::Up(Coordinates { x, y }, button.from_mouse_code());
            event_handler_mouse_up.event(event);
        };

        js! {
            var element = @{&*self};
            var mouseDown = @{mouse_down};
            var mouseUp = @{mouse_up};

            element.addEventListener("mousedown", function(event) {
                mouseDown(event.clientX, event.clientY, event.button);
            });

            element.addEventListener("mouseup", function(event) {
                mouseUp(event.clientX, event.clientY, event.button);
            });
        }
    }
}
