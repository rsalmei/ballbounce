#[macro_use]
mod colors;
mod ball;
mod board;
mod game;

use game::Game;
use std::thread;
use std::time::{Duration, Instant};

const FRAMES_PER_SECOND: u32 = 30;
const SKIP_TICKS: u32 = 1000 / FRAMES_PER_SECOND;

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

    let mut start;
    let mut sleep_time;
    loop {
        start = Instant::now();
        // process input.
        game.tick(); // update game.
        print!("{}", game); // display game.

        sleep_time = SKIP_TICKS as i32 - start.elapsed().as_millis() as i32;
        if sleep_time >= 0 {
            thread::sleep(Duration::from_millis(sleep_time as u64));
        }
        println!("\x1b[{}A", game.board.size.1 + 2);
    }
}
