#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RotationIndex {
    index: usize,
    nr_rotations: usize,
}

//TODO: remove allow dead_code when RotationIndex is used by application code
impl RotationIndex {
    pub fn new(value: usize, nr_rotations: usize) -> Self {
        Self {
            index: value,
            nr_rotations,
        }
    }

    pub fn rotate_clockwise(&mut self) {
        self.index = (self.index + 1) % self.nr_rotations
    }

    pub fn rotate_counterclockwise(&mut self) {
        self.index = (self.index + self.nr_rotations - 1) % self.nr_rotations
    }
}

impl From<RotationIndex> for usize {
    fn from(rotation: RotationIndex) -> usize {
        rotation.index
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rotation_index_rotate_clockwise_advances_index() {
        // Arrange
        let mut rotation = RotationIndex::new(0, 4);

        // Act
        rotation.rotate_clockwise();

        // Assert
        assert_eq!(usize::from(rotation), 1);
    }

    #[test]
    fn rotation_index_rotate_clockwise_wraps_at_max() {
        // Arrange
        let mut rotation = RotationIndex::new(3, 4);

        // Act
        rotation.rotate_clockwise();

        // Assert
        assert_eq!(usize::from(rotation), 0);
    }

    #[test]
    fn rotation_index_rotate_counterclockwise_decreases_index() {
        // Arrange
        let mut rotation = RotationIndex::new(2, 4);

        // Act
        rotation.rotate_counterclockwise();

        // Assert
        assert_eq!(usize::from(rotation), 1);
    }

    #[test]
    fn rotation_index_rotate_counterclockwise_wraps_at_zero() {
        // Arrange
        let mut rotation = RotationIndex::new(0, 4);

        // Act
        rotation.rotate_counterclockwise();

        // Assert
        assert_eq!(usize::from(rotation), 3);
    }
}
