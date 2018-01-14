use std::collections::HashSet;
use std::sync::{Arc, Mutex};

use events::{Event, Coordinates, Button};
use block::{Block, Child};
use super::ffi::{self, AtomId, Attribute, EventType};
use super::{Candidate, Rendered, Update};

/// A wrapper to interface with FFI atoms.
pub struct Atom {
    id: AtomId,
    content: Rendered,
}

impl Drop for Atom {
    fn drop(&mut self) {
        ffi::delete_node(self.id);
    }
}

impl Atom {
    pub fn mount() -> Self {
        Self {
            id: AtomId::root(),

            // The mount element.
            content: Rendered::Element {
                registered_events: HashSet::new(),
                attributes: vec![
                    Attribute::new("id", ffi::mount_id())
                ],
                children: vec![],
            }
        }
    }

    /// Construct a new node under a parent node.
    fn new<U>(candidate: Candidate<U::Message>, parent: AtomId, update: U) -> Self
    where
        U: Update,
    {
        match candidate {
            Candidate::Text(text) => {
                let id = ffi::create_text_node(&text, parent);

                Self {
                    id,
                    content: Rendered::Text(text)
                }
            }

            Candidate::Element { children, attributes, event_handler } => {
                let event_handler = Arc::new(Mutex::new(event_handler));
                let id = ffi::create_element(attributes.clone(), parent);

                let children = children
                    .into_iter()
                    .map(|candidate| Atom::new(candidate, id, update.clone()))
                    .collect();

                let mut registered_events = HashSet::new();

                let test_event = event_handler.lock().unwrap();

                if test_event.event(Event::Click(Coordinates { x: 0, y: 0 })).is_some() {
                    registered_events.insert(EventType::Click);

                    let event_handler = event_handler.clone();
                    let update = update.clone();

                    #[derive(Deserialize)]
                    struct Data {
                        x: u32,
                        y: u32,
                    }

                    ffi::create_event(id, EventType::Click, move |Data { x, y }: Data| {
                        let guard = event_handler.lock().unwrap();
                        if let Some(msg) = guard.event(Event::Click(Coordinates { x, y })) {
                            update.reduce(msg);
                        }
                    });
                } else if test_event.event(Event::MouseDown(Coordinates { x: 0, y: 0 }, Button::Left)).is_some() {
                    registered_events.insert(EventType::MouseDown);

                    let event_handler = event_handler.clone();
                    let update = update.clone();

                    #[derive(Deserialize)]
                    struct Data {
                        button: u32,
                        x: u32,
                        y: u32,
                    }

                    ffi::create_event(id, EventType::MouseDown, move |Data { x, y, button }: Data| {
                        let guard = event_handler.lock().unwrap();
                        let button = match button {
                            0 => Button::Left,
                            1 => Button::Middle,
                            2 => Button::Right,
                            _ => return,
                        };

                        if let Some(msg) = guard.event(Event::MouseDown(Coordinates { x, y }, button)) {
                            update.reduce(msg);
                        }
                    });
                }

                Atom {
                    id,
                    content: Rendered::Element {
                        attributes,
                        children,
                        registered_events,
                    }
                }
            }
        }
    }

    /// Upgrade the current node to match the candidate tree.
    ///
    /// This implements the tree diffing algorithm.
    pub fn upgrade<U>(&mut self, candidate: Candidate<U::Message>, update: U)
    where
        U: Update,
    {
        // First we move the updated data out of the candidate tree. Then, we
        // use different ref patterns depending on how we need to update the
        // content. This is to avoid bind by-move and by-ref in the same
        // pattern.
        match candidate {
            Candidate::Text(new_text) => {
                match &mut self.content {
                    &mut Rendered::Text(ref mut old_text) => {
                        if *old_text != new_text {
                            ffi::update_text_node(new_text.clone(), self.id);
                            *old_text = new_text;
                        }
                    }

                    ref mut element @ &mut Rendered::Element { .. } => {
                        **element = Rendered::Text(new_text.clone());
                        ffi::element_to_text_node(self.id, new_text);
                    }
                }
            }

            Candidate::Element {
                attributes: new_attributes,
                children: mut new_children,
                event_handler,
            } => {
                match &mut self.content {
                    ref mut element @ &mut Rendered::Text(..) => {
                        ffi::text_node_to_element(self.id, new_attributes.clone());

                        let id = self.id;
                        let new_children = new_children
                            .into_iter()
                            .map(|candidate| Atom::new(candidate, id, update.clone()))
                            .collect();

                        **element = Rendered::Element {
                            attributes: new_attributes,
                            children: new_children,
                            registered_events: HashSet::new(),
                        };
                    }

                    &mut Rendered::Element { ref mut children, ref mut attributes, .. } => {
                        ffi::update_element(self.id, new_attributes.clone());
                        *attributes = new_attributes;

                        let num_nodes = children.len();

                        if num_nodes > new_children.len() {
                            // Drop the old nodes.
                            let _ = children.drain(num_nodes - 1..);
                        }

                        // The new children array.
                        let mut updated_children = Vec::new();

                        // Push the existing children and upgrade the nodes.
                        for _ in 0..num_nodes {
                            let mut node = children.remove(0);
                            node.upgrade(new_children.remove(0), update.clone());
                            updated_children.push(node);
                        }

                        // Create new nodes from any remaining additional children
                        for _ in num_nodes..new_children.len() {
                            let atom = Atom::new(new_children.remove(0), self.id, update.clone());
                            updated_children.push(atom);
                        }

                        *children = updated_children;
                    }
                }
            }
        }
    }
}
