//TODO: remove allow dead_code when RotationIndex is used by application code
#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub struct RotationIndex {
    index: usize,
}

//TODO: remove allow dead_code when RotationIndex is used by application code
#[allow(dead_code)]
impl RotationIndex {
    pub fn new(value: usize) -> Self {
        Self { index: value }
    }
}

impl From<RotationIndex> for usize {
    fn from(rotation: RotationIndex) -> usize {
        rotation.index
    }
}
