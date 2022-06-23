use crate::engine::game_object::GameObject;
use crate::position::Position;

#[derive(Debug)]
pub struct Tree {
    pub position: Position,
}

impl Tree {
    pub fn new(position: Position) -> Self {
        Tree { position }
    }
}

impl GameObject for Tree {
    fn get_position(&self) -> Position {
        self.position
    }

    fn get_char_representation(&self) -> char {
        'T'
    }

    fn is_tree(&self) -> bool {
        true
    }
}
