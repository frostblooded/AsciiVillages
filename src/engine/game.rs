use crate::engine::game_object::GameObject;
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
        // Create a copy of the grid so that we can keep the old grid
        // while we update the cells.
        let new_grid = self.world.grid.clone();

        for cell in new_grid.iter().flatten() {
            cell.update(&mut self.world);
        }

        self.world.grid = new_grid;
    }

    pub fn draw(&self) {
        self.world.print();
        println!();
    }
}
