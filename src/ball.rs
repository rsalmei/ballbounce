use crate::board::Board;
use crate::colors::Style;
use crate::game::FrameBuffer;
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

impl Ball {
    const REPRS: &'static str = "●◉❖▲✢✦★❤";

    pub fn new(board: &Board) -> Ball {
        let mut rng = rand::thread_rng();
        let r = |i: usize| rand::random::<f32>() * i as f32; // WOW, I couldn't find a way to use `rng` here.
        let v = || r(4) - 2.;
        Ball {
            position: (r(board.size.0), r(board.size.1)),
            velocity: (v(), v()),
            color: rng.gen(),
            repr: Ball::REPRS.chars().choose(&mut rng).unwrap(),
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
