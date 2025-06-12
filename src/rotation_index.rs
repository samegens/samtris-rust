//TODO: remove allow dead_code when RotationIndex is used by application code
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RotationIndex {
    value: usize,
}

//TODO: remove allow dead_code when RotationIndex is used by application code
#[allow(dead_code)]
impl RotationIndex {
    pub fn new(value: usize) -> Self {
        Self { value }
    }

    pub fn value(&self) -> usize {
        self.value
    }
}
