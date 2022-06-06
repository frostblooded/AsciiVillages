use crate::world::World;

pub trait GameObject {
    fn update(&mut self, world: &mut World) {}
}
