mod component;
mod frame_buffer;
mod world;

use crate::ball::{Ball, BallBuilder};
use crate::board::Board;
use crate::colors::Style;
use crate::domain::Size;
pub use component::Component;
pub use frame_buffer::FrameBuffer;
use std::fmt::Write as _Write;
use std::io::{self, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use termion::clear;
use termion::cursor;
use termion::event::Key;
pub use world::World;

const FRAMES_PER_SECOND: f32 = 30.;
const SKIP_TICKS: i64 = (1000. / FRAMES_PER_SECOND) as i64;

pub struct Game {
    world: World,
    board: Board,
    balls: Vec<Ball>,
    frame_buffer: FrameBuffer,
}

impl Game {
    pub fn new(size: Size, num_balls: usize) -> Game {
        let size = Size {
            w: size.w,
            h: size.h - 1,
        };
        let world = World::new(size);
        let board = Board::new(size);

        let mut balls = vec![BallBuilder::new()
            .with_color(Style::Red)
            .with_repr('◉')
            .build(&world)];
        BallBuilder::new().extend(num_balls - 1, &mut balls, &world);

        let capacity = board.size_hint() + balls.iter().map(Component::size_hint).sum::<u16>();
        let frame_buffer = FrameBuffer::new(capacity);
        Game {
            world,
            board,
            balls,
            frame_buffer,
        }
    }

    pub fn run(&mut self, stdout: &mut impl Write, rx: mpsc::Receiver<Key>) -> io::Result<()> {
        let mut info = String::with_capacity(128);

        loop {
            let start = Instant::now();
            match rx.try_recv() {
                Ok(Key::Ctrl('c')) | Ok(Key::Char('q')) | Err(TryRecvError::Disconnected) => {
                    return Ok(());
                }
                Ok(k) => self.process_input(k),
                Err(TryRecvError::Empty) => {}
            }
            let input_end = Instant::now();
            self.process_update();
            let update_end = Instant::now();
            self.process_render(stdout)?;
            let render_end = Instant::now();
            let frame_time = (render_end - start).as_millis(); // TODO how to sample the average `frame_time` per second?

            write!(
                info,
                "balls: {}  frame_time: {:2} ({:2} {:2} {:2})  ↑ +ball  ↓ -ball  ← shuffle balls  → shuffle board",
                self.balls.len(),
                frame_time,
                (input_end - start).as_millis(),
                (update_end - input_end).as_millis(),
                (render_end - update_end).as_millis(),
            ).unwrap(); // write! to a String returns fmt::Error.
            write!(
                stdout,
                "{}{:.*}{}",
                cursor::Goto(1, self.world.size.h + 1),
                self.world.size.w as usize,
                info,
                clear::UntilNewline,
            )?; // write! to a Write returns io::Error.
            info.clear();
            stdout.flush()?;

            let sleep_time = SKIP_TICKS - frame_time as i64;
            if sleep_time >= 0 {
                thread::sleep(Duration::from_millis(sleep_time as u64));
            }
        }
    }

    fn process_input(&mut self, key: Key) {
        match key {
            Key::Up => BallBuilder::new().extend(1, &mut self.balls, &self.world),
            Key::Down => {
                if self.balls.len() > 1 {
                    self.balls.pop();
                }
            }
            Key::Left => {
                let old = self.balls.drain(1..).collect::<Vec<_>>();
                old.iter().for_each(|b| {
                    BallBuilder::new()
                        .with_position(b.position)
                        .with_velocity(b.velocity)
                        .extend(1, &mut self.balls, &self.world)
                })
            }
            Key::Right => {
                let old = self.balls.drain(1..).collect::<Vec<_>>();
                old.iter().for_each(|b| {
                    BallBuilder::new()
                        .with_position(b.position)
                        .with_color(b.color)
                        .with_repr(b.repr)
                        .extend(1, &mut self.balls, &self.world)
                })
            }
            _ => {}
        }
    }

    fn process_update(&mut self) {
        self.board.update(&self.world);
        for c in self.balls.iter_mut() {
            c.update(&self.world)
        }
    }

    fn process_render(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.board.draw_to(&mut self.frame_buffer);
        for c in self.balls.iter() {
            c.draw_to(&mut self.frame_buffer)
        }

        self.frame_buffer.render_to(stdout)
    }
}
