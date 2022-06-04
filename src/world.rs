use crate::cell::Cell;
use crate::position::Position;
use itertools::Itertools;
use rand::prelude::IteratorRandom;
use rand::seq::SliceRandom;
use rand::Rng;

pub struct World {
    grid: Vec<Vec<Cell>>,
    grid_size: usize,
}

impl World {
    pub fn new(grid_size: usize) -> Self {
        World {
            grid: vec![vec![Cell::Empty; grid_size]; grid_size],
            grid_size,
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

    pub fn initialize(&mut self, bases_count: u32) {
        self.spawn_bases(bases_count);
        self.spawn_trees();
        self.spawn_workers();
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

    fn spawn_workers(&mut self) {
        const WORKERS_PER_BASE: usize = 2;

        for base_position in self.get_positions_for_cell_of_type(Cell::Base) {
            const INCLUDE_MIDDLE: bool = false;
            let chosen_workers_positions: Vec<Position> = self
                .get_adjacent_cells_positions(base_position, INCLUDE_MIDDLE)
                .into_iter()
                .filter(|&position| self.get(position).is_some())
                .choose_multiple(&mut rand::thread_rng(), WORKERS_PER_BASE);

            for chosen_worker_pos in chosen_workers_positions {
                if let Some(cell) = self.get_mut(chosen_worker_pos) {
                    *cell = Cell::Worker;
                }
            }
        }
    }

    fn get_positions_for_cell_of_type(&self, cell_type: Cell) -> Vec<Position> {
        self.get_grid_positions()
            .into_iter()
            .filter(|&position| {
                self.get(position)
                    .map(|cell| *cell == cell_type)
                    .unwrap_or(false)
            })
            .collect()
    }

    fn get_grid_positions(&self) -> Vec<Position> {
        (0..self.grid_size)
            .cartesian_product(0..self.grid_size)
            .map(|(x, y)| Position::new(x, y))
            .collect()
    }

    fn spawn_bases(&mut self, bases_count: u32) {
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

    fn spawn_trees(&mut self) {
        for y in 0..self.grid_size {
            for x in 0..self.grid_size {
                let position = Position::new(x, y);

                if self.is_cell_good_for_tree(position) {
                    self.set_cell(position, Cell::Tree);
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

    fn get_adjacent_cells(&self, position: Position, include_middle: bool) -> Vec<&Cell> {
        self.get_adjacent_cells_positions(position, include_middle)
            .iter()
            .filter_map(|&p| self.get(p))
            .collect()
    }

    fn is_cell_good_for_base(&self, position: Position) -> bool {
        const INCLUDE_MIDDLE: bool = true;
        self.get_adjacent_cells(position, INCLUDE_MIDDLE)
            .iter()
            .all(|&cell| *cell == Cell::Empty)
    }

    fn is_cell_good_for_tree(&self, position: Position) -> bool {
        const INCLUDE_MIDDLE: bool = true;
        self.get_adjacent_cells(position, INCLUDE_MIDDLE)
            .iter()
            .all(|&cell| *cell == Cell::Empty || *cell == Cell::Tree)
    }
}
