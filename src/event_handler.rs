use std::collections::HashMap;

use crate::InfalliableResult;

#[derive(Hash, Eq, PartialEq)]
pub enum Event {
    SpaceKeyUp,
}

pub struct EventHandler {
    events: HashMap<Event, Box<dyn Fn() -> InfalliableResult>>,
}

impl EventHandler {
    pub fn new() -> EventHandler {
        EventHandler {
            events: HashMap::new(),
        }
    }

    pub fn register_event(&mut self, event: Event, action: Box<dyn Fn() -> InfalliableResult>) {
        self.events.insert(event, action);
    }

    pub fn handle_event(&self, event: Event) -> InfalliableResult {
        self.events.get(&event).map_or(Ok(()), |action| action())
    }
}
