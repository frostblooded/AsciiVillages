use crate::engine::game_object::GameObject;
use crate::position::Position;

#[derive(Debug)]
pub struct Base {
    pub position: Position,
}

impl Base {
    pub fn new(position: Position) -> Self {
        Base { position }
    }
}

impl GameObject for Base {
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_char_representation(&self) -> char {
        'B'
    }

    fn is_base(&self) -> bool {
        true
    }
}
