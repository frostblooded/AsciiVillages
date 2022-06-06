use super::cell_type::CellType;
use crate::engine::game_object::GameObject;
use crate::position::Position;

#[derive(Clone)]
pub struct Cell {
    pub position: Position,
    pub cell_type: CellType,
}

impl Cell {
    pub fn new(position: Position, cell_type: CellType) -> Self {
        Cell {
            position,
            cell_type,
        }
    }
}

impl GameObject for Cell {}
