#[macro_use]
mod colors;
mod ball;
mod board;
mod game;
mod utils;

use game::Game;
use std::io::{self, stdin, stdout, Write};
use std::thread;
use std::time::{Duration, Instant};
use termion::cursor::Goto;
use termion::raw::IntoRawMode;
use termion::{clear, terminal_size};
use utils::Size;

const FRAMES_PER_SECOND: u32 = 30;
const SKIP_TICKS: i64 = 1000 / FRAMES_PER_SECOND as i64;
fn main() -> io::Result<()> {
    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}", clear::All)?;

    let size = terminal_size().map_or(Size { w: 100, h: 30 }, |(w, h)| Size { w, h });
    let mut game = Game::new(
        Size {
            w: size.w,
            h: size.h - 1,
        },
        20,
    );

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

        write!(
            stdout,
            "{}balls: {}  frame_time: {:2} ({:2} {:2} {:2})",
            Goto(1, size.h),
            game.balls.len(),
            frame_time,
            (input_end - start).as_millis(),
            (update_end - input_end).as_millis(),
            (render_end - update_end).as_millis(),
        )?;
        stdout.flush()?;

        let sleep_time = SKIP_TICKS - frame_time as i64;
        if sleep_time >= 0 {
            thread::sleep(Duration::from_millis(sleep_time as u64));
        }
    }
    Ok(())
}
