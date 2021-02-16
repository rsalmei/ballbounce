#[macro_use]
mod colors;
mod ball;
mod board;
mod game;

use game::Game;
use std::thread;
use std::time::Duration;

fn main() {
    let mut game = Game::new(5);
    println!(
        "{}",
        game.balls
            .iter()
            .enumerate()
            .map(|(i, b)| format!("{}: {}", i + 1, b))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let delay = Duration::from_secs_f32(1. / 30.);
    loop {
        print!("{}", game);
        thread::sleep(delay);
        game.tick();
        println!("\x1b[{}A", game.board.size.1 + 2);
    }
}
