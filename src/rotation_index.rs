#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RotationIndex {
    value: usize,
}

impl RotationIndex {
    pub fn new(value: usize) -> Self {
        Self { value }
    }
    
    pub fn value(&self) -> usize {
        self.value
    }
}
