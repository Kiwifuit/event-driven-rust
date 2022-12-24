use std::collections::HashMap;
use std::path::PathBuf;

pub type EventListenerCallback = fn(file_path: PathBuf);

#[derive(Eq, Hash, PartialEq, Clone)]
pub enum Event {
    Save,
    Load,
    Reload,
}

#[derive(Default)]
pub struct EventPublisher {
    events: HashMap<Event, Vec<EventListenerCallback>>,
}

impl EventPublisher {
    pub fn subscribe(&mut self, event: Event, listener: EventListenerCallback) {
        self.events.entry(event.clone()).or_default();
        self.events.get_mut(&event).unwrap().push(listener);
    }

    pub fn unsubscribe(&mut self, event: Event, listener: EventListenerCallback) {
        self.events
            .get_mut(&event)
            .unwrap()
            .retain(|&l| l != listener);
    }

    pub fn notify(&self, event: Event, path: &PathBuf) {
        let subscribers = self.events.get(&event).unwrap();

        for subscriber in subscribers {
            subscriber(path.clone());
        }
    }
}
