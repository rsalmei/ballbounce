#[macro_use]
mod colors;
mod ball;
mod board;
mod component;
mod domain;
mod frame_buffer;
mod game;

use domain::Size;
use game::Game;
use std::io::{self, stdin, stdout, Write};
use std::sync::mpsc::{self};
use std::thread;
use termion::cursor;
use termion::input::TermRead;
use termion::raw::IntoRawMode;
use termion::{clear, terminal_size};

fn main() -> io::Result<()> {
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || {
        for c in stdin().keys() {
            tx.send(c.unwrap()).unwrap();
        }
    });

    let mut stdout = stdout().into_raw_mode().unwrap();
    write!(stdout, "{}{}", clear::All, cursor::Hide)?;

    let size = terminal_size().map_or(Size { w: 100, h: 30 }, |(w, h)| Size { w, h });
    let mut game = Game::new(size, 20);

    // main game loop.
    game.run(&mut stdout, rx)?;

    write!(stdout, "{}", cursor::Show)?;
    Ok(())
}
