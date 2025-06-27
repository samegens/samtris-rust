use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::events::Event;

pub struct EventQueue {
    queue: Arc<Mutex<VecDeque<Event>>>,
}

impl EventQueue {
    pub fn new() -> Self {
        Self {
            queue: Arc::new(Mutex::new(VecDeque::new())),
        }
    }

    pub fn push_back(&self, event: Event) {
        let mut queue = self.queue.lock().unwrap();
        queue.push_back(event);
    }

    pub fn drain(&self) -> Vec<Event> {
        let mut queue = self.queue.lock().unwrap();
        queue.drain(..).collect()
    }

    #[cfg(test)]
    pub fn assert_contains(&self, expected: Event) {
        assert!(
            self.queue.lock().unwrap().contains(&expected),
            "Event {:?} not found in queued events: {:?}",
            expected,
            self.queue.lock().unwrap()
        );
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn new_creates_empty_event_bus() {
        // Act
        let sut = EventQueue::new();

        // Assert
        let queue = sut.queue.lock().unwrap();
        assert!(queue.is_empty());
    }

    #[test]
    fn publish_queues_event_without_immediate_processing() {
        // Arrange
        let sut = EventQueue::new();

        // Act
        sut.push_back(Event::LevelStarted(5));

        // Assert
        let queue = sut.queue.lock().unwrap();
        assert_eq!(queue.len(), 1);
    }

    #[test]
    #[should_panic(expected = "Event LevelStarted(5) not found in queued events")]
    fn assert_contains_panics_when_event_not_found() {
        // Arrange
        let sut = EventQueue::new();

        // Act & Assert - should panic
        sut.assert_contains(Event::LevelStarted(5));
    }
}
