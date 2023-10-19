use crate::ball::{Ball, BallBuilder};
use crate::board::Board;
use crate::color;
use crate::component::Component;
use crate::domain::Size;
use crate::frame_buffer::FrameBuffer;
use std::fmt::Write as _Write;
use std::io::{self, Write};
use std::sync::mpsc::{self, TryRecvError};
use std::thread;
use std::time::{Duration, Instant};
use termion::clear;
use termion::cursor;
use termion::event::Key;

const FRAMES_PER_SECOND: f32 = 30.;
const SKIP_TICKS: i64 = (1000. / FRAMES_PER_SECOND) as i64;

pub struct Game {
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
        let board = Board::new(size);

        let mut balls = vec![BallBuilder::new()
            .with_color(color::RED)
            .with_repr('◉')
            .build_one(&board)];
        BallBuilder::new().build_multiple(num_balls - 1, &mut balls, &board);

        let capacity = board.size_hint() + balls.iter().map(Component::size_hint).sum::<u16>();
        let frame_buffer = FrameBuffer::new(capacity);
        Game {
            board,
            balls,
            frame_buffer,
        }
    }

    pub fn run(&mut self, stdout: &mut impl Write, rx: mpsc::Receiver<Key>) -> io::Result<()> {
        let mut info = String::with_capacity(128);

        let (mut frame, start) = (1u64, Instant::now());
        loop {
            let start_frame = Instant::now();
            match rx.try_recv() {
                Ok(Key::Ctrl('c')) | Ok(Key::Char('q')) | Err(TryRecvError::Disconnected) => {
                    return Ok(());
                }
                Ok(k) => self.process_input(k),
                Err(TryRecvError::Empty) => {}
            }
            self.process_update();
            self.process_render(stdout)?;
            let frame_time = start_frame.elapsed().as_millis(); // TODO how to sample the average `frame_time` per second?

            write!(
                info,
                "balls: {}  fps: {:3.0} ({:2}ms)    ↑ +ball  ↓ -ball  ← shuffle lables  → shuffle balls",
                self.balls.len(),
                frame as f64 / start.elapsed().as_secs_f64(),
                frame_time,
            ).unwrap(); // write! to a String returns fmt::Error.
            write!(
                stdout,
                "{}{:.*}{}",
                cursor::Goto(1, self.board.size.h + 1),
                self.board.size.w as usize,
                info,
                clear::UntilNewline,
            )?; // write! to a Write returns io::Error.
            info.clear();
            stdout.flush()?;

            let sleep_time = SKIP_TICKS - frame_time as i64;
            if sleep_time >= 0 {
                thread::sleep(Duration::from_millis(sleep_time as u64));
            }
            frame += 1;
        }
    }

    fn process_input(&mut self, key: Key) {
        match key {
            Key::Up => BallBuilder::new().build_multiple(1, &mut self.balls, &self.board),
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
                        .build_multiple(1, &mut self.balls, &self.board)
                })
            }
            Key::Right => {
                let old = self.balls.drain(1..).collect::<Vec<_>>();
                old.iter().for_each(|b| {
                    BallBuilder::new()
                        .with_position(b.position)
                        .with_color(b.color)
                        .with_repr(b.repr)
                        .build_multiple(1, &mut self.balls, &self.board)
                })
            }
            _ => {}
        }
    }

    fn process_update(&mut self) {
        for c in self.balls.iter_mut() {
            c.update(&self.board)
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
