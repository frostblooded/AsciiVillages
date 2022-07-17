mod base_component;
mod drawable_component;
mod position_component;
mod tree_component;
mod worker_component;

use base_component::BaseComponent;
use bevy::prelude::*;
use drawable_component::DrawableComponent;
use position_component::{get_distance, Position, PositionComponent};
use tree_component::TreeComponent;

use rand::{prelude::IteratorRandom, Rng};
use worker_component::WorkerComponent;

const GRID_SIZE: u32 = 8;
const WORKERS_PER_BASE: u32 = 2;

fn get_random_position() -> Position {
    let mut rng = rand::thread_rng();

    Position::new(rng.gen_range(0..GRID_SIZE), rng.gen_range(0..GRID_SIZE))
}

fn get_adjacent_positions(position: Position) -> Vec<Position> {
    let x = position.x as i32;
    let y = position.y as i32;

    vec![
        (x - 1, y - 1),
        (x, y - 1),
        (x + 1, y - 1),
        (x - 1, y),
        (x + 1, y),
        (x - 1, y + 1),
        (x, y + 1),
        (x + 1, y + 1),
    ]
    .into_iter()
    .filter_map(|(signed_x, signed_y)| {
        if signed_x >= 0
            && signed_x < GRID_SIZE as i32
            && signed_y >= 0
            && signed_y < GRID_SIZE as i32
        {
            Some(Position::new(signed_x as u32, signed_y as u32))
        } else {
            None
        }
    })
    .collect()
}

fn spawn_world_system(mut commands: Commands) {
    let base_position: Position = get_random_position();

    commands
        .spawn()
        .insert(BaseComponent)
        .insert(PositionComponent::new(base_position))
        .insert(DrawableComponent::new('B'));

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let position = Position::new(x, y);

            if get_distance(position, base_position) > 1 {
                commands
                    .spawn()
                    .insert(TreeComponent)
                    .insert(PositionComponent::new(position))
                    .insert(DrawableComponent::new('T'));
            }
        }
    }

    let worker_positions: Vec<Position> = get_adjacent_positions(base_position)
        .into_iter()
        .choose_multiple(&mut rand::thread_rng(), WORKERS_PER_BASE as usize);

    for worker_position in worker_positions {
        commands
            .spawn()
            .insert(WorkerComponent)
            .insert(PositionComponent::new(worker_position))
            .insert(DrawableComponent::new('W'));
    }
}

fn draw_system(query: Query<(&DrawableComponent, &PositionComponent)>) {
    let mut grid_draw_cache: Vec<char> = vec![' '; (GRID_SIZE * GRID_SIZE) as usize];

    for (drawable_component, position_component) in query.iter() {
        let position = position_component.position;
        let index: usize = (position.y * GRID_SIZE + position.x) as usize;
        grid_draw_cache[index] = drawable_component.symbol;
    }

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let index: usize = (y * GRID_SIZE + x) as usize;
            print!("|{}", grid_draw_cache[index]);
        }

        println!("|");
    }
}

fn main() {
    let mut app: App = App::new();
    app.add_startup_system(spawn_world_system);
    app.add_system(draw_system);
    app.run();
}
