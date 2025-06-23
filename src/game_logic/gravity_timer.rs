use std::time::Duration;

const GRAVITY_FRAMES: [u64; 21] = [
    53, 49, 45, 41, 37, 33, 28, 22, 17, 11, 10, 9, 8, 7, 6, 6, 5, 5, 4, 4, 3,
];
const FRAME_DURATION_MS: u64 = 16; // ~60 FPS

pub struct GravityTimer {
    time_since_last_drop: Duration,
    level: u32,
}

// TODO: remove allow dead code when all functions from GravityTimer are used by game.
#[allow(dead_code)]
impl GravityTimer {
    pub fn new(level: u32) -> Self {
        Self {
            time_since_last_drop: Duration::ZERO,
            level: Self::cap_level(level),
        }
    }

    fn cap_level(level: u32) -> u32 {
        level.min(GRAVITY_FRAMES.len() as u32 - 1)
    }

    /// Update with delta_time, returns true if the update should trigger moving the tetromino
    /// one line down.
    pub fn update(&mut self, delta_time: Duration) -> bool {
        self.time_since_last_drop += delta_time;
        let interval = self.get_interval();

        if self.time_since_last_drop >= interval {
            self.time_since_last_drop = Duration::ZERO;
            true
        } else {
            false
        }
    }

    pub fn set_level(&mut self, level: u32) {
        self.level = Self::cap_level(level);
    }

    pub fn get_level(&self) -> u32 {
        self.level
    }

    pub fn reset(&mut self) {
        self.time_since_last_drop = Duration::ZERO;
    }

    fn get_interval(&self) -> Duration {
        let frames = GRAVITY_FRAMES[self.level as usize];
        Duration::from_millis(frames * FRAME_DURATION_MS)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::time::Duration;

    #[test]
    fn new_gravity_timer_has_correct_level() {
        // Arrange & Act
        let sut = GravityTimer::new(5);

        // Assert
        assert_eq!(sut.get_level(), 5);
        assert_eq!(sut.time_since_last_drop, Duration::ZERO);
    }

    #[test]
    fn new_gravity_timer_caps_level_at_maximum() {
        // Arrange & Act
        let sut = GravityTimer::new(100); // Way above max

        // Assert
        assert_eq!(sut.get_level(), GRAVITY_FRAMES.len() as u32 - 1);
    }

    #[test]
    fn update_returns_false_when_timer_has_not_reached_interval() {
        // Arrange
        let mut sut = GravityTimer::new(0); // Level 0: 53 frames = 848ms

        // Act
        let result = sut.update(Duration::from_millis(400));

        // Assert
        assert!(!result);
        assert_eq!(sut.time_since_last_drop, Duration::from_millis(400));
    }

    #[test]
    fn update_returns_true_when_timer_reaches_interval() {
        // Arrange
        let mut sut = GravityTimer::new(0); // Level 0: 53 frames = 848ms
        let interval = Duration::from_millis(53 * FRAME_DURATION_MS);

        // Act
        let result = sut.update(interval);

        // Assert
        assert!(result);
        assert_eq!(sut.time_since_last_drop, Duration::ZERO);
    }

    #[test]
    fn update_returns_true_when_timer_exceeds_interval() {
        // Arrange
        let mut sut = GravityTimer::new(0); // Level 0: 53 frames = 848ms
        let interval = Duration::from_millis(53 * FRAME_DURATION_MS + 100);

        // Act
        let result = sut.update(interval);

        // Assert
        assert!(result);
        assert_eq!(sut.time_since_last_drop, Duration::ZERO);
    }

    #[test]
    fn update_accumulates_time_across_multiple_calls() {
        // Arrange
        let mut sut = GravityTimer::new(0); // Level 0: 53 frames = 848ms
        assert!(!sut.update(Duration::from_millis(300)));
        assert!(!sut.update(Duration::from_millis(400)));

        // Act
        let result = sut.update(Duration::from_millis(200));

        // Assert
        assert!(result); // 300 + 400 + 200 = 900ms > 848ms
    }

    #[test]
    fn reset_clears_accumulated_time() {
        // Arrange
        let mut sut = GravityTimer::new(0);
        sut.update(Duration::from_millis(500));

        // Act
        sut.reset();

        // Assert
        assert_eq!(sut.time_since_last_drop, Duration::ZERO);
        assert!(!sut.update(Duration::from_millis(300))); // Should not trigger yet
    }

    #[test]
    fn set_level_changes_trigger_time() {
        // Arrange
        let mut sut = GravityTimer::new(0); // Level 0: 53 frames = 848ms

        // Act
        sut.set_level(20); // Level 20: 3 frames = 48ms
        let result = sut.update(Duration::from_millis(50));

        // Assert
        assert!(result); // Should trigger with much shorter interval
        assert_eq!(sut.get_level(), 20);
    }

    #[test]
    fn set_level_caps_at_maximum() {
        // Arrange
        let mut sut = GravityTimer::new(0);

        // Act
        sut.set_level(100); // Way above max

        // Assert
        assert_eq!(sut.get_level(), GRAVITY_FRAMES.len() as u32 - 1);
    }

    #[test]
    fn set_level_preserves_accumulated_time() {
        // Arrange
        let mut sut = GravityTimer::new(0); // Level 0: 53 frames = 848ms
        sut.update(Duration::from_millis(500));

        // Act
        sut.set_level(20); // Level 20: 3 frames = 48ms
        let result = sut.update(Duration::from_millis(1));

        // Assert
        assert!(result); // 500ms already accumulated > 48ms new interval
    }

    #[test]
    fn get_interval_returns_correct_duration_for_different_levels() {
        // Arrange
        let sut_level_0 = GravityTimer::new(0);
        let sut_level_20 = GravityTimer::new(20);

        // Act & Assert
        assert_eq!(
            sut_level_0.get_interval(),
            Duration::from_millis(53 * FRAME_DURATION_MS)
        );
        assert_eq!(
            sut_level_20.get_interval(),
            Duration::from_millis(3 * FRAME_DURATION_MS)
        );
    }
}
