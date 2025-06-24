use crate::events::{Event, EventBus, EventType};
use std::sync::{Arc, Mutex};

pub struct TestEventBusWrapper {
    event_bus: Arc<EventBus>,
    captured_events: Arc<Mutex<Vec<Event>>>,
}

impl TestEventBusWrapper {
    pub fn new() -> Self {
        let event_bus = Arc::new(EventBus::new());
        let captured_events = Arc::new(Mutex::new(Vec::new()));

        // Subscribe to all event types we want to capture
        let capture_clone = captured_events.clone();
        event_bus.subscribe(EventType::LevelStarted, move |event| {
            capture_clone.lock().unwrap().push(event.clone());
        });

        let capture_clone2 = captured_events.clone();
        event_bus.subscribe(EventType::LinesCleared, move |event| {
            capture_clone2.lock().unwrap().push(event.clone());
        });

        Self {
            event_bus,
            captured_events,
        }
    }

    pub fn get_event_bus(&self) -> Arc<EventBus> {
        self.event_bus.clone()
    }

    pub fn get_captured_events(&self) -> Vec<Event> {
        self.captured_events.lock().unwrap().clone()
    }

    pub fn assert_event_published(&self, expected: Event) {
        let events = self.get_captured_events();
        assert!(
            events.contains(&expected),
            "Event {:?} not found in captured events: {:?}",
            expected,
            events
        );
    }
}
