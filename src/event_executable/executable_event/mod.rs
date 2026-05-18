use aion_event::prelude::Event;

pub struct ExecutableEvent {
    id: String,
    event: Event
}

impl ExecutableEvent {
    pub fn id(&self) -> &String {
        &self.id
    }

    pub fn event(&self) -> &Event {
        &self.event
    }
}