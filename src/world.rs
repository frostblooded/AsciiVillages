use crate::cell::Cell;
use crate::cell_type::CellType;
use crate::position::Position;
use rand::prelude::IteratorRandom;
use rand::Rng;

pub struct World {
    pub grid: Vec<Vec<Cell>>,
}

impl World {
    pub fn new(grid_size: usize) -> Self {
        let mut grid: Vec<Vec<Cell>> = vec![vec![]; grid_size];

        for y in 0..grid_size {
            let row = &mut grid[y];

            for x in 0..grid_size {
                row.push(Cell::new(Position::new(x, y), CellType::Empty));
            }
        }

        World { grid }
    }

    pub fn print(&self) {
        for row in &self.grid {
            for cell in row {
                print!("|{}", cell.cell_type);
            }

            println!("|");
        }
    }

    pub fn initialize(&mut self) {
        self.spawn_base();
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

    pub fn set_cell_type(&mut self, position: Position, new_cell_type: CellType) {
        if let Some(cell) = self.get_mut(position) {
            cell.cell_type = new_cell_type;
        }
    }

    pub fn get_base(&self) -> &Cell {
        let bases = self.get_cells_of_type(CellType::Base);
        assert_eq!(bases.len(), 1);
        bases[0]
    }

    fn spawn_workers(&mut self) {
        const WORKERS_PER_BASE: usize = 2;

        const INCLUDE_MIDDLE: bool = false;
        let chosen_worker_cells_positions: Vec<Position> = self
            .get_adjacent_cells_positions(self.get_base().position, INCLUDE_MIDDLE)
            .into_iter()
            .choose_multiple(&mut rand::thread_rng(), WORKERS_PER_BASE);

        for worker_cell_position in chosen_worker_cells_positions {
            self.set_cell_type(worker_cell_position, CellType::Worker);
        }
    }

    fn get_cells_of_type(&self, cell_type: CellType) -> Vec<&Cell> {
        self.grid
            .iter()
            .flatten()
            .filter(|cell| cell.cell_type == cell_type)
            .collect()
    }

    fn spawn_base(&mut self) {
        self.set_cell_type(self.get_random_position(), CellType::Base);
    }

    fn spawn_trees(&mut self) {
        for y in 0..self.grid.len() {
            for x in 0..self.grid.len() {
                let position = Position::new(x, y);

                if self.is_cell_good_for_tree(position) {
                    self.set_cell_type(position, CellType::Tree);
                }
            }
        }
    }

    fn get_random_position(&self) -> Position {
        let mut rng = rand::thread_rng();
        Position::new(
            rng.gen_range(0..self.grid.len()),
            rng.gen_range(0..self.grid.len()),
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
            .into_iter()
            .filter_map(|pos| self.get(pos))
            .collect()
    }

    fn is_cell_good_for_tree(&self, position: Position) -> bool {
        const INCLUDE_MIDDLE: bool = true;
        self.get_adjacent_cells(position, INCLUDE_MIDDLE)
            .iter()
            .all(|&cell| cell.cell_type == CellType::Empty || cell.cell_type == CellType::Tree)
    }
}
