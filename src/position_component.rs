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

pub fn get_length(position: Position) -> u32 {
    ((position.x * position.x + position.y * position.y) as f32)
        .sqrt()
        .floor() as u32
}

pub fn abs_substract_positions(position1: Position, position2: Position) -> Position {
    Position::new(
        (position1.x as i32 - position2.x as i32).unsigned_abs(),
        (position1.y as i32 - position2.y as i32).unsigned_abs(),
    )
}

pub fn get_distance(position1: Position, position2: Position) -> u32 {
    get_length(abs_substract_positions(position1, position2))
}
