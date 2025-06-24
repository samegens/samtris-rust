use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use crate::events::{Event, EventType};

type EventHandler = Box<dyn Fn(&Event) + Send + Sync>;

pub struct EventBus {
    handlers: Arc<Mutex<HashMap<EventType, Vec<EventHandler>>>>,
}

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
        }
    }

    // TODO allow once subscribe is actually used
    #[allow(dead_code)]
    pub fn subscribe<F>(&self, event_type: EventType, handler: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.lock().unwrap();
        handlers
            .entry(event_type)
            .or_default()
            .push(Box::new(handler));
    }

    pub fn publish(&self, event: Event) {
        let handlers = self.handlers.lock().unwrap();
        let event_type = self.get_event_type(&event);

        if let Some(event_handlers) = handlers.get(&event_type) {
            for handler in event_handlers {
                handler(&event);
            }
        }
    }

    fn get_event_type(&self, event: &Event) -> EventType {
        match event {
            Event::LevelStarted(_) => EventType::LevelStarted,
            Event::LinesCleared(_) => EventType::LinesCleared,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    #[test]
    fn new_creates_empty_event_bus() {
        // Act
        let sut = EventBus::new();

        // Assert
        let handlers = sut.handlers.lock().unwrap();
        assert!(handlers.is_empty());
    }

    #[test]
    fn subscribe_adds_handler_for_event_type() {
        // Arrange
        let sut = EventBus::new();
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        // Act
        sut.subscribe(EventType::LevelStarted, move |_| {
            *called_clone.lock().unwrap() = true;
        });

        // Assert
        let handlers = sut.handlers.lock().unwrap();
        assert_eq!(handlers.len(), 1);
        assert!(handlers.contains_key(&EventType::LevelStarted));
    }

    #[test]
    fn publish_calls_registered_handler() {
        // Arrange
        let sut = EventBus::new();
        let received_level = Arc::new(Mutex::new(None));
        let received_clone = received_level.clone();

        sut.subscribe(EventType::LevelStarted, move |event| {
            if let Event::LevelStarted(level) = event {
                *received_clone.lock().unwrap() = Some(*level);
            }
        });

        // Act
        sut.publish(Event::LevelStarted(5));

        // Assert
        let level = *received_level.lock().unwrap();
        assert_eq!(level, Some(5));
    }

    #[test]
    fn publish_with_no_subscribers_does_nothing() {
        // Arrange
        let sut = EventBus::new();

        // Act
        sut.publish(Event::LevelStarted(3));

        // Assert
        let handlers = sut.handlers.lock().unwrap();
        assert!(handlers.is_empty());
    }

    #[test]
    fn multiple_subscribers_all_receive_event() {
        // Arrange
        let sut = EventBus::new();
        let call_count = Arc::new(Mutex::new(0));

        let count_clone1 = call_count.clone();
        sut.subscribe(EventType::LevelStarted, move |_| {
            *count_clone1.lock().unwrap() += 1;
        });

        let count_clone2 = call_count.clone();
        sut.subscribe(EventType::LevelStarted, move |_| {
            *count_clone2.lock().unwrap() += 1;
        });

        // Act
        sut.publish(Event::LevelStarted(1));

        // Assert
        let count = *call_count.lock().unwrap();
        assert_eq!(count, 2);
    }
}
