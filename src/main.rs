mod base;
mod engine;
mod position;
mod tree;
mod worker;
mod world;

use std::time::Duration;

use engine::game::Game;

fn main() {
    let mut game = Game::new();

    loop {
        game.update();
        game.draw();

        std::thread::sleep(Duration::from_secs(1));
    }
}
