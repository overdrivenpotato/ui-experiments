use std::collections::HashMap;
use std::cell::RefCell;
use std::sync::Mutex;

use super::{AtomId, EventType};

type Callback = Box<Fn(String) + Send>;
type EventMap = HashMap<EventType, Callback>;

lazy_static! {
    static ref EVENTS: Mutex<RefCell<HashMap<AtomId, EventMap>>> = {
        Mutex::new(RefCell::new(HashMap::new()))
    };
}

pub fn call(type_: EventType, atom: AtomId, json_data: String) {
    let guard = EVENTS.lock().unwrap();
    let map = guard.borrow();

    if let Some(event_map) = map.get(&atom) {
        if let Some(handler) = event_map.get(&type_) {
            handler(json_data);
        }
    }
}

pub fn create_event<F>(id: AtomId, type_: EventType, callback: F)
where
    F: 'static + Send + Fn(String),
{
    let callback = Box::new(callback) as Callback;
    let guard = EVENTS.lock().unwrap();
    let mut atom_map = guard.borrow_mut();

    if let Some(event_map) = atom_map.get_mut(&id) {
        event_map.insert(type_, callback);

        // Early return to avoid LL error.
        return
    }

    let mut event_map = HashMap::new();
    event_map.insert(type_, callback);
    atom_map.insert(id, event_map);
}
