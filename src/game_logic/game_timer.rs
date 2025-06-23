use std::time::{Duration, Instant};

pub struct GameTimer {
    last_time: Instant,
}

impl GameTimer {
    pub fn new() -> Self {
        Self {
            last_time: Instant::now(),
        }
    }

    pub fn delta(&mut self) -> Duration {
        let current_time = Instant::now();
        let delta_time = current_time - self.last_time;
        self.last_time = current_time;
        delta_time
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::thread;
    use std::time::Duration;

    #[test]
    fn delta_returns_approximately_correct_duration() {
        // Arrange
        let mut sut = GameTimer::new();

        // Act
        thread::sleep(Duration::from_millis(5));
        let delta = sut.delta();

        // Assert
        // Allow some tolerance for timing variations
        assert!(delta >= Duration::from_millis(4));
        assert!(delta <= Duration::from_millis(10));
    }

    #[test]
    fn consecutive_deltas_accumulate_time() {
        // Arrange
        let mut sut = GameTimer::new();

        // Act
        thread::sleep(Duration::from_millis(3));
        let delta1 = sut.delta();
        thread::sleep(Duration::from_millis(3));
        let delta2 = sut.delta();

        // Assert
        assert!(delta1 >= Duration::from_millis(2));
        assert!(delta2 >= Duration::from_millis(2));
    }
}
