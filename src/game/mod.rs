mod frame_buffer;

use crate::ball::{Ball, BallBuilder};
use crate::board::Board;
use crate::colors::Style;
use crate::utils::Size;
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
    pub fn new(size: Size, mut num_balls: usize) -> Game {
        let board = Board::new(size);

        num_balls = num_balls.max(1).min(Ball::COMBINATIONS);
        let mut balls = Vec::with_capacity(num_balls);
        balls.push(
            BallBuilder::new()
                .with_color(Style::RED)
                .with_repr('◉')
                .build(&board),
        );
        while balls.len() < num_balls {
            balls.push(loop {
                let candidate = BallBuilder::new().build(&board);
                if balls.iter().all(|b| &candidate != b) {
                    break candidate;
                }
            });
        }

        let frame_buffer = FrameBuffer::new(&board.size);
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
