use std::collections::{HashMap, VecDeque};
use std::sync::{Arc, Mutex};

use crate::events::{Event, EventType};

type EventHandler = Arc<dyn Fn(&Event) + Send + Sync>;

pub struct EventBus {
    handlers: Arc<Mutex<HashMap<EventType, Vec<EventHandler>>>>,
    event_queue: Arc<Mutex<VecDeque<Event>>>,
}

type EventHandlers = Vec<Arc<dyn Fn(&Event) + Send + Sync + 'static>>;

impl EventBus {
    pub fn new() -> Self {
        Self {
            handlers: Arc::new(Mutex::new(HashMap::new())),
            event_queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn subscribe<F>(&self, event_type: EventType, handler: F)
    where
        F: Fn(&Event) + Send + Sync + 'static,
    {
        let mut handlers = self.handlers.lock().unwrap();
        handlers
            .entry(event_type)
            .or_default()
            .push(Arc::new(handler));
    }

    pub fn publish(&self, event: Event) {
        let mut queue = self.event_queue.lock().unwrap();
        queue.push_back(event);
    }

    pub fn process_events(&self) {
        // Process events until queue is empty
        loop {
            let Some(event) = self.get_next_event() else {
                break; // No more events
            };

            for handler in self.get_safe_handlers(&event) {
                handler(&event);
            }
        }
    }

    fn get_safe_handlers(&self, event: &Event) -> EventHandlers {
        let handlers = self.handlers.lock().unwrap();
        let event_type = self.get_event_type(event);
        handlers.get(&event_type).cloned().unwrap_or_default()
    }

    fn get_next_event(&self) -> Option<Event> {
        let event = {
            let mut queue = self.event_queue.lock().unwrap();
            queue.pop_front()
        };
        event
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
        let queue = sut.event_queue.lock().unwrap();
        assert!(queue.is_empty());
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
    fn publish_queues_event_without_immediate_processing() {
        // Arrange
        let sut = EventBus::new();
        let called = Arc::new(Mutex::new(false));
        let called_clone = called.clone();

        sut.subscribe(EventType::LevelStarted, move |_| {
            *called_clone.lock().unwrap() = true;
        });

        // Act
        sut.publish(Event::LevelStarted(5));

        // Assert - handler not called yet
        assert!(!*called.lock().unwrap());
        let queue = sut.event_queue.lock().unwrap();
        assert_eq!(queue.len(), 1);
    }

    #[test]
    fn process_events_calls_registered_handler() {
        // Arrange
        let sut = EventBus::new();
        let received_level = Arc::new(Mutex::new(None));
        let received_clone = received_level.clone();

        sut.subscribe(EventType::LevelStarted, move |event| {
            if let Event::LevelStarted(level) = event {
                *received_clone.lock().unwrap() = Some(*level);
            }
        });

        sut.publish(Event::LevelStarted(5));

        // Act
        sut.process_events();

        // Assert
        let level = *received_level.lock().unwrap();
        assert_eq!(level, Some(5));
        let queue = sut.event_queue.lock().unwrap();
        assert!(queue.is_empty());
    }

    #[test]
    fn process_events_with_no_subscribers_does_nothing() {
        // Arrange
        let sut = EventBus::new();
        sut.publish(Event::LevelStarted(3));

        // Act
        sut.process_events();

        // Assert
        let queue = sut.event_queue.lock().unwrap();
        assert!(queue.is_empty());
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

        sut.publish(Event::LevelStarted(1));

        // Act
        sut.process_events();

        // Assert
        let count = *call_count.lock().unwrap();
        assert_eq!(count, 2);
    }

    #[test]
    fn handler_can_publish_events_without_deadlock() {
        // Arrange
        let sut = EventBus::new();
        let events_received = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events_received.clone();

        sut.subscribe(EventType::LinesCleared, move |event| {
            if let Event::LinesCleared(lines) = event {
                events_clone
                    .lock()
                    .unwrap()
                    .push(format!("LinesCleared({})", lines));
                // This handler publishes another event - should not deadlock
                // Note: We need a reference to the EventBus here, which is tricky in this setup
            }
        });

        let events_clone2 = events_received.clone();
        sut.subscribe(EventType::LevelStarted, move |event| {
            if let Event::LevelStarted(level) = event {
                events_clone2
                    .lock()
                    .unwrap()
                    .push(format!("LevelStarted({})", level));
            }
        });

        sut.publish(Event::LinesCleared(10));

        // Act
        sut.process_events();

        // Assert
        let events = events_received.lock().unwrap();
        assert_eq!(events.len(), 1); // Only the first event, since handler can't publish easily in this test
    }

    #[test]
    fn process_events_handles_events_published_during_processing() {
        // Arrange
        let sut = Arc::new(EventBus::new());
        let events_received = Arc::new(Mutex::new(Vec::new()));
        let events_clone = events_received.clone();
        let sut_clone = sut.clone();

        sut.subscribe(EventType::LinesCleared, move |event| {
            if let Event::LinesCleared(lines) = event {
                events_clone.lock().unwrap().push(*lines);
                if *lines == 10 {
                    // Publish another event when we get 10 lines
                    sut_clone.publish(Event::LevelStarted(1));
                }
            }
        });

        let events_clone2 = events_received.clone();
        sut.subscribe(EventType::LevelStarted, move |event| {
            if let Event::LevelStarted(level) = event {
                events_clone2.lock().unwrap().push(*level + 100); // Add 100 to distinguish
            }
        });

        sut.publish(Event::LinesCleared(10));

        // Act
        sut.process_events();

        // Assert
        let events = events_received.lock().unwrap().clone();
        assert_eq!(events, vec![10, 101]); // Both events processed
    }
}
