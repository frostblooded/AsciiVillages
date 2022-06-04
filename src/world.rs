use crate::cell::Cell;
use crate::position::Position;
use rand::Rng;

pub struct World {
    grid: Vec<Vec<Cell>>,
    size: usize,
}

impl World {
    pub fn new(grid_size: usize) -> Self {
        World {
            grid: vec![vec![Cell::Empty; grid_size]; grid_size],
            size: grid_size,
        }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for cell in row {
                print!("|{}", cell);
            }

            println!("|");
        }
    }

    pub fn spawn_bases(&mut self, bases_count: u32) {
        for _ in 0..bases_count {
            // Loop until we find a position for this base
            loop {
                let position = self.get_random_position();

                if self.is_cell_good_for_base(position) {
                    self.set_cell(position, Cell::Base);
                    break;
                }
            }
        }
    }

    pub fn get(&self, position: Position) -> Option<&Cell> {
        self.grid.get(position.y).and_then(|r| r.get(position.x))
    }

    pub fn get_mut(&mut self, position: Position) -> Option<&mut Cell> {
        self.grid
            .get_mut(position.y)
            .and_then(|r| r.get_mut(position.x))
    }

    pub fn set_cell(&mut self, position: Position, new_cell: Cell) {
        if let Some(cell) = self.get_mut(position) {
            *cell = new_cell;
        }
    }

    fn get_random_position(&self) -> Position {
        let mut rng = rand::thread_rng();
        Position::new(rng.gen_range(0..self.size), rng.gen_range(0..self.size))
    }

    // Checks if all nearby (surrounding + current) cells are empty
    fn is_cell_good_for_base(&self, position: Position) -> bool {
        let x: i64 = position.x.try_into().expect("usize too big for i64");
        let y: i64 = position.y.try_into().expect("usize too big for i64");

        let positions_to_check: Vec<(i64, i64)> = vec![
            (x - 1, y - 1),
            (x, y - 1),
            (x + 1, y - 1),
            (x - 1, y),
            (x, y),
            (x + 1, y),
            (x - 1, y + 1),
            (x, y + 1),
            (x + 1, y + 1),
        ];

        // Check if the positions are empty
        positions_to_check.iter().all(|&signed_position| {
            let usize_x = signed_position.0.try_into();
            let usize_y = signed_position.1.try_into();

            // Return false if one of the indices is out of range (couldn't be cast into usize)
            if usize_x.is_err() || usize_y.is_err() {
                return false;
            }

            let checked_position = Position::new(usize_x.unwrap(), usize_y.unwrap());

            self.get(checked_position)
                .map(|cell| *cell == Cell::Empty)
                .unwrap_or(true)
        })
    }
}
