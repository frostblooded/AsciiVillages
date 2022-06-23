use crate::engine::game_object::GameObject;
use crate::position::Position;

#[derive(Debug)]
pub struct Worker {
    pub position: Position,
}

impl Worker {
    pub fn new(position: Position) -> Self {
        Worker { position }
    }
}

impl GameObject for Worker {
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_char_representation(&self) -> char {
        'W'
    }
}
