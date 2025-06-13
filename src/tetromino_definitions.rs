use crate::tetromino_definition::TetrominoDefinition;
use crate::tetromino_type::TetrominoType;
use std::collections::HashMap;

pub struct TetrominoDefinitions {
    definitions: HashMap<TetrominoType, TetrominoDefinition>,
}

//TODO: remove allow dead_code when TetrominoDefinitions is used by application code
#[allow(dead_code)]
impl TetrominoDefinitions {
    pub fn new() -> Self {
        let mut definitions = HashMap::new();
        definitions.insert(TetrominoType::I, TetrominoDefinition::create_i());
        definitions.insert(TetrominoType::O, TetrominoDefinition::create_o());
        definitions.insert(TetrominoType::T, TetrominoDefinition::create_t());
        definitions.insert(TetrominoType::Z, TetrominoDefinition::create_z());
        definitions.insert(TetrominoType::S, TetrominoDefinition::create_s());
        definitions.insert(TetrominoType::J, TetrominoDefinition::create_j());
        definitions.insert(TetrominoType::L, TetrominoDefinition::create_l());

        Self { definitions }
    }

    pub fn get(&self, tetromino_type: TetrominoType) -> &TetrominoDefinition {
        &self.definitions[&tetromino_type]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(TetrominoType::I)]
    #[case(TetrominoType::O)]
    #[case(TetrominoType::T)]
    #[case(TetrominoType::Z)]
    #[case(TetrominoType::S)]
    #[case(TetrominoType::J)]
    #[case(TetrominoType::L)]
    fn tetromino_definitions_returns_correct_type(#[case] tetromino_type: TetrominoType) {
        // Arrange
        let definitions = TetrominoDefinitions::new();

        // Act
        let definition = definitions.get(tetromino_type);

        // Assert
        assert_eq!(definition.get_type(), tetromino_type);
    }
}
