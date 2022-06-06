mod cell;
mod cell_type;
mod engine;
mod position;
mod world;

use world::World;

const WORLD_SIZE: usize = 10;

fn main() {
    let mut world = World::new(WORLD_SIZE);
    world.initialize();
    world.print();
}
