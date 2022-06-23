use crate::base::Base;
use crate::engine::game_object::GameObject;
use crate::position::Position;
use crate::tree::Tree;
use crate::worker::Worker;
use rand::prelude::IteratorRandom;
use rand::Rng;

pub struct World {
    pub objects: Vec<Box<dyn GameObject>>,
    pub grid_size: usize,
}

impl World {
    pub fn new(grid_size: usize) -> Self {
        World {
            objects: vec![],
            grid_size,
        }
    }

    pub fn print(&self) {
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                if let Some(object) = self.get_object_at_position(Position::new(x, y)) {
                    print!("|{}", object.get_char_representation());
                } else {
                    print!("| ");
                }
            }

            println!("|");
        }
    }

    pub fn initialize(&mut self) {
        self.spawn_base();
        self.spawn_trees();
        self.spawn_workers();
    }

    fn get_object_at_position(&self, position: Position) -> Option<&Box<dyn GameObject>> {
        for object in &self.objects {
            if object.get_position() == position {
                return Some(object);
            }
        }

        None
    }

    fn add_object(&mut self, object: Box<dyn GameObject>) {
        self.objects.push(object);
    }

    fn spawn_base(&mut self) {
        self.add_object(Box::new(Base::new(self.get_random_position())));
    }

    fn get_base(&self) -> Option<&Box<dyn GameObject>> {
        for object in &self.objects {
            if object.is_base() {
                return Some(object);
            }
        }

        None
    }

    fn spawn_workers(&mut self) {
        const WORKERS_PER_BASE: usize = 2;

        let base = self
            .get_base()
            .expect("Base doesn't exist. Please spawn a base before spawning the workers.");

        const INCLUDE_MIDDLE: bool = false;
        let chosen_worker_cells_positions: Vec<Position> = self
            .get_adjacent_cells_positions(base.get_position(), INCLUDE_MIDDLE)
            .into_iter()
            .choose_multiple(&mut rand::thread_rng(), WORKERS_PER_BASE);

        for worker_cell_position in chosen_worker_cells_positions {
            self.add_object(Box::new(Worker::new(worker_cell_position)));
        }
    }

    fn spawn_trees(&mut self) {
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let position = Position::new(x, y);

                if self.is_cell_good_for_tree(position) {
                    self.add_object(Box::new(Tree::new(position)));
                }
            }
        }
    }

    fn get_random_position(&self) -> Position {
        let mut rng = rand::thread_rng();
        Position::new(
            rng.gen_range(0..self.grid_size),
            rng.gen_range(0..self.grid_size),
        )
    }

    fn get_adjacent_cells_positions(
        &self,
        position: Position,
        include_middle: bool,
    ) -> Vec<Position> {
        let x: i64 = position.x.try_into().expect("usize too big for i64");
        let y: i64 = position.y.try_into().expect("usize too big for i64");

        let mut positions = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        if include_middle {
            positions.push((x, y));
        }

        positions
            .into_iter()
            .filter_map(|(signed_x, signed_y)| {
                if let (Ok(ux), Ok(uy)) = (signed_x.try_into(), signed_y.try_into()) {
                    Some(Position::new(ux, uy))
                } else {
                    // Return false if one of the indices is out of range (couldn't be cast into usize)
                    None
                }
            })
            .collect()
    }

    fn get_adjacent_cells_objects(
        &self,
        position: Position,
        include_middle: bool,
    ) -> Vec<&Box<dyn GameObject>> {
        self.get_adjacent_cells_positions(position, include_middle)
            .into_iter()
            .filter_map(|pos| self.get_object_at_position(pos))
            .collect()
    }

    fn is_cell_good_for_tree(&self, position: Position) -> bool {
        const INCLUDE_MIDDLE: bool = true;

        self.get_adjacent_cells_objects(position, INCLUDE_MIDDLE)
            .iter()
            .all(|o| o.is_tree())
    }
}
