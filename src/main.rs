mod cell;
mod position;
mod world;

use crate::world::World;

const WORLD_SIZE: usize = 10;
const BASES_COUNT: u32 = 3;

fn main() {
    let mut world = World::new(WORLD_SIZE);
    world.spawn_bases(BASES_COUNT);
    world.print();
}
