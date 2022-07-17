use bevy::prelude::Component;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Position {
    pub x: u32,
    pub y: u32,
}

impl Position {
    pub fn new(x: u32, y: u32) -> Self {
        Self { x, y }
    }
}

#[derive(Component, Debug)]
pub struct PositionComponent {
    pub position: Position,
}

impl PositionComponent {
    pub fn new(position: Position) -> Self {
        Self { position }
    }
}

pub fn are_positions_adjacent(position1: Position, position2: Position) -> bool {
    let x_dist = (position1.x as i32 - position2.x as i32).unsigned_abs();
    let y_dist = (position1.y as i32 - position2.y as i32).unsigned_abs();

    x_dist <= 1 && y_dist <= 1
}
