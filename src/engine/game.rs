use crate::world::World;

pub struct Game {
    world: World,
}

impl Game {
    pub fn new() -> Self {
        const WORLD_SIZE: usize = 10;

        let mut world = World::new(WORLD_SIZE);
        world.initialize();

        Game { world }
    }

    pub fn update(&mut self) {
        for object in self.world.objects.iter_mut() {
            // object.update(&mut self.world)
        }
    }

    pub fn draw(&self) {
        self.world.print();
        println!();
    }
}
