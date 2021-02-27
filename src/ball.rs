use crate::board::Board;
use crate::colors::Style;
use crate::game::FrameBuffer;
use rand::rngs::ThreadRng;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Ball {
    position: (f32, f32),
    velocity: (f32, f32),
    pub color: Style,
    pub repr: char,
}

#[derive(Default)]
pub struct BallBuilder {
    color: Option<Style>,
    repr: Option<char>,
}

impl BallBuilder {
    pub fn with_color(&mut self, color: Style) -> &mut BallBuilder {
        self.color = Some(color);
        self
    }

    pub fn with_repr(&mut self, repr: char) -> &mut BallBuilder {
        self.repr = Some(repr);
        self
    }

    pub fn build(&self, board: &Board) -> Ball {
        Ball::build(board, self.color, self.repr)
    }
}

impl Ball {
    const REPRS: &'static str = "●◉❖▲✢✦★❤";

    pub fn new() -> BallBuilder {
        BallBuilder::default()
    }

    fn build(board: &Board, color: Option<Style>, repr: Option<char>) -> Ball {
        let mut rng = rand::thread_rng();
        let r = |i: usize, rng: &mut ThreadRng| rng.gen::<f32>() * i as f32;
        let v = |rng: &mut ThreadRng| r(4, rng) - 2.;
        Ball {
            position: (r(board.size.0, &mut rng), r(board.size.1, &mut rng)),
            velocity: (v(&mut rng), v(&mut rng)),
            color: color.unwrap_or_else(|| rng.gen()),
            repr: repr.unwrap_or_else(|| *Ball::REPRS.chars().choose(&mut rng).unwrap()),
        }
    }

    pub fn update(&mut self, board: &Board) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        if self.position.0 < 0. || self.position.0 >= board.size.0 as f32 {
            self.velocity.0 = -self.velocity.0;
            self.position.0 += self.velocity.0
        }
        if self.position.1 < 0. || self.position.1 >= board.size.1 as f32 {
            self.velocity.1 = -self.velocity.1;
            self.position.1 += self.velocity.1
        }
    }

    pub fn actual_pos(&self) -> (usize, usize) {
        (self.position.0 as usize, self.position.1 as usize)
    }

    pub fn draw_to(&self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.draw(self.actual_pos(), self.color, self.repr);
    }
}

impl Display for Ball {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", style!(self.color, self.repr))?;
        write!(f, "{}{:?}", style!(Style::DIM, "\tposition"), self.position)?;
        write!(f, "{}{:?}", style!(Style::DIM, "\tvelocity"), self.velocity)
    }
}

impl PartialEq for Ball {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.repr == other.repr
    }
}
