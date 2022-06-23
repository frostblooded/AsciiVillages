use crate::world::World;

pub trait GameObject {
    fn update(&self, _world: &World) {}
}
