mod frame_buffer;

use crate::ball::Ball;
use crate::board::Board;
use crate::colors::Style;
pub use frame_buffer::FrameBuffer;
use itertools::Itertools;
use std::fmt::{self, Display, Formatter};

pub struct Game {
    pub board: Board,
    pub balls: Vec<Ball>,
    frame_buffer: FrameBuffer,
    frame_buffer_back: FrameBuffer,
}

impl Game {
    pub fn new(num_balls: usize) -> Game {
        let board = Board::new();
        let balls = (0..num_balls).map(|_| Ball::new(&board)).collect();
        let frame_buffer = FrameBuffer::new(board.size);
        Game {
            board,
            balls,
            frame_buffer: frame_buffer.clone(),
            frame_buffer_back: frame_buffer,
        }
    }

    pub fn process_input(&mut self) {}

    pub fn update(&mut self) {
        for b in self.balls.iter_mut() {
            b.update(&self.board)
        }
    }

    pub fn render(&mut self) {
        self.frame_buffer_back.clear();
        self.board.draw_to(&mut self.frame_buffer_back);
        for b in self.balls.iter() {
            b.draw_to(&mut self.frame_buffer_back)
        }

        std::mem::swap(&mut self.frame_buffer, &mut self.frame_buffer_back);
        print!("{}", self);
    }

    pub fn caption(&self) -> String {
        self.balls
            .iter()
            .enumerate()
            .map(|(i, b)| format!("{}: {}", i + 1, b))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let border = |f: &mut Formatter<'_>| {
            write!(f, "{}+", Style::RED)?;
            for _ in 0..self.board.size.0 {
                write!(f, "-")?;
            }
            write!(f, "+{}", Style::RESET)
        };

        border(f)?;
        write!(
            f,
            "\n{}\n",
            self.frame_buffer.iter().format_with("\n", |row, f| {
                f(&format_args!(
                    "{}{}{}",
                    style!(Style::RED, "|"),
                    &row,
                    style!(Style::RED, "|")
                ))
            })
        )?;
        border(f)
    }
}
