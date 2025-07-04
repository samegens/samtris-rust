use crate::events::{Event, EventQueue};
use std::sync::Arc;

pub struct LevelManager {
    current_level: u32,
    total_lines_cleared: u32,
    score: u32,
    event_bus: Arc<EventQueue>,
}

impl LevelManager {
    pub fn new(event_bus: Arc<EventQueue>) -> Self {
        Self {
            current_level: 0,
            total_lines_cleared: 0,
            score: 0,
            event_bus,
        }
    }

    fn calculate_level(total_lines_cleared: u32) -> u32 {
        total_lines_cleared / 10
    }

    pub fn get_score(&self) -> u32 {
        self.score
    }

    fn calculate_score(lines_cleared: u32, level: u32) -> u32 {
        let base_points = match lines_cleared {
            1 => 40,
            2 => 100,
            3 => 300,
            4 => 1200,
            _ => 0,
        };
        base_points * (level + 1)
    }

    pub fn get_current_level(&self) -> u32 {
        self.current_level
    }

    pub fn get_total_lines_cleared(&self) -> u32 {
        self.total_lines_cleared
    }

    pub fn start_level(&mut self, level: u32) {
        self.current_level = level;
        self.event_bus.push_back(Event::LevelStarted(level));
    }

    pub fn handle_lines_cleared(&mut self, nr_lines: u32) {
        let points = Self::calculate_score(nr_lines, self.current_level);
        self.score += points;

        self.total_lines_cleared += nr_lines;

        let new_level = Self::calculate_level(self.total_lines_cleared);

        if new_level > self.current_level {
            self.current_level = new_level;
            self.event_bus.push_back(Event::LevelStarted(new_level));
        }
    }
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[test]
    fn new_level_manager_starts_at_level_zero() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());

        // Act
        let sut = LevelManager::new(event_bus);

        // Assert
        assert_eq!(sut.get_current_level(), 0);
        assert_eq!(sut.get_total_lines_cleared(), 0);
    }

    #[test]
    fn calculate_level_returns_correct_level_for_lines_cleared() {
        // Arrange & Act & Assert
        assert_eq!(LevelManager::calculate_level(0), 0);
        assert_eq!(LevelManager::calculate_level(9), 0);
        assert_eq!(LevelManager::calculate_level(10), 1);
        assert_eq!(LevelManager::calculate_level(19), 1);
        assert_eq!(LevelManager::calculate_level(20), 2);
        assert_eq!(LevelManager::calculate_level(155), 15);
    }

    #[test]
    fn lines_cleared_event_updates_level_when_threshold_reached() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus.clone());

        // Act
        sut.handle_lines_cleared(5);
        sut.handle_lines_cleared(3);
        sut.handle_lines_cleared(2); // Total: 10 lines

        // Assert
        assert_eq!(sut.get_current_level(), 1);
        assert_eq!(sut.get_total_lines_cleared(), 10);
    }

    #[test]
    fn lines_cleared_event_does_not_update_level_when_threshold_not_reached() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus.clone());

        // Act
        sut.handle_lines_cleared(3);
        sut.handle_lines_cleared(4); // Total: 7 lines

        // Assert
        assert_eq!(sut.get_current_level(), 0);
        assert_eq!(sut.get_total_lines_cleared(), 7);
    }

    #[test]
    fn level_increase_publishes_level_started_event() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus.clone());

        // Act
        sut.handle_lines_cleared(10);

        // Assert
        assert_eq!(sut.get_current_level(), 1);
    }

    #[test]
    fn multiple_level_increases_work_correctly() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus.clone());

        // Act
        sut.handle_lines_cleared(15);
        sut.handle_lines_cleared(8);
        sut.handle_lines_cleared(7);

        // Assert
        assert_eq!(sut.get_current_level(), 3);
        assert_eq!(sut.get_total_lines_cleared(), 15 + 8 + 7);
    }

    #[test]
    fn start_level_sets_level_and_publishes_event() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus.clone());

        // Act
        sut.start_level(5);

        // Assert
        assert_eq!(sut.get_current_level(), 5);
        event_bus.assert_contains(Event::LevelStarted(5));
    }

    #[test]
    fn level_does_not_decrease_on_subsequent_events() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus.clone());
        sut.handle_lines_cleared(15);

        // Act
        sut.handle_lines_cleared(1);

        // Assert
        assert_eq!(sut.get_current_level(), 1);
        assert_eq!(sut.get_total_lines_cleared(), 16);
    }

    #[test]
    fn start_level_overrides_calculated_level() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus.clone());

        sut.handle_lines_cleared(25);

        // Act
        sut.start_level(5);

        // Assert
        assert_eq!(sut.get_current_level(), 5);
        assert_eq!(sut.get_total_lines_cleared(), 25);
        event_bus.assert_contains(Event::LevelStarted(5));
    }

    #[test]
    fn level_continues_to_increase_after_manual_start_level() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus.clone());
        sut.start_level(3);

        // Act
        sut.handle_lines_cleared(15);
        sut.handle_lines_cleared(25);

        // Assert
        assert_eq!(sut.get_current_level(), 4);
        assert_eq!(sut.get_total_lines_cleared(), 40);
    }

    #[test]
    fn new_level_manager_starts_with_zero_score() {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());

        // Act
        let sut = LevelManager::new(event_bus);

        // Assert
        assert_eq!(sut.get_score(), 0);
    }

    #[rstest]
    #[case(1, 0, 40)] // 1 line at level 0: 40 * (0+1) = 40
    #[case(2, 0, 100)] // 2 lines at level 0: 100 * (0+1) = 100
    #[case(3, 0, 300)] // 3 lines at level 0: 300 * (0+1) = 300
    #[case(4, 0, 1200)] // 4 lines at level 0: 1200 * (0+1) = 1200
    #[case(1, 1, 80)] // 1 line at level 1: 40 * (1+1) = 80
    #[case(4, 2, 3600)] // 4 lines at level 2: 1200 * (2+1) = 3600
    fn handle_lines_cleared_calculates_correct_score(
        #[case] lines_cleared: u32,
        #[case] level: u32,
        #[case] expected_score: u32,
    ) {
        // Arrange
        let event_bus = Arc::new(EventQueue::new());
        let mut sut = LevelManager::new(event_bus);
        sut.start_level(level);

        // Act
        sut.handle_lines_cleared(lines_cleared);

        // Assert
        assert_eq!(sut.get_score(), expected_score);
    }
}
