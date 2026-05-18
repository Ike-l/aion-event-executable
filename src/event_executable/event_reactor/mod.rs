use std::collections::HashSet;

use aion_event::prelude::Event;

pub struct EventReactor { 
    events: HashSet<Event> 
}

impl EventReactor {
    pub fn events(&self) -> impl Iterator<Item = &Event> {
        self.events.iter()
    }
}