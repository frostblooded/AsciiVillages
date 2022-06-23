use std::fmt::Debug;

use crate::{position::Position, world::World};

pub trait GameObject: Debug {
    fn update(&self, _world: &mut World) {}
    fn get_position(&self) -> Position;
    fn get_char_representation(&self) -> char;

    fn is_base(&self) -> bool {
        false
    }

    fn is_tree(&self) -> bool {
        false
    }
}
