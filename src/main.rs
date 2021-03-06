#[macro_use]
mod colors;
mod ball;
mod board;
mod game;
mod utils;

use game::Game;
use std::thread;
use std::time::{Duration, Instant};

const FRAMES_PER_SECOND: u32 = 30;
const SKIP_TICKS: u32 = 1000 / FRAMES_PER_SECOND;

fn main() {
    let mut game = Game::new(20);
    loop {
        let start = Instant::now();
        // main game loop.
        game.process_input();
        game.update();
        game.render();
        let frame_time = start.elapsed(); // TODO how to sample the average `frame_time` per second?

        let sleep_time = SKIP_TICKS as i64 - frame_time.as_millis() as i64;
        if sleep_time >= 0 {
            thread::sleep(Duration::from_millis(sleep_time as u64));
        }
        println!(
            "{:2}\x1b[{}A",
            frame_time.as_millis(),
            game.board.size.1 + 2
        );
    }
}
