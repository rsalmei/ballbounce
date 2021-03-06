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
const SKIP_TICKS: i64 = 1000 / FRAMES_PER_SECOND as i64;

fn main() {
    let mut game = Game::new(20);
    loop {
        let start = Instant::now();
        // main game loop.
        game.process_input();
        let input_instant = Instant::now();
        game.update();
        let update_instant = Instant::now();
        game.render();
        let render_instant = Instant::now();
        let frame_time = (render_instant - start).as_millis(); // TODO how to sample the average `frame_time` per second?

        let sleep_time = SKIP_TICKS - frame_time as i64;
        if sleep_time >= 0 {
            thread::sleep(Duration::from_millis(sleep_time as u64));
        }
        println!(
            "{:2} ({:2} {:2} {:2})\x1b[{}A",
            frame_time,
            (input_instant - start).as_millis(),
            (update_instant - input_instant).as_millis(),
            (render_instant - update_instant).as_millis(),
            game.board.size.1 + 2
        );
    }
}
