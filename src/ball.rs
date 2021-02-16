use crate::board::Board;
use crate::colors::Color;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Ball {
    position: (f32, f32),
    velocity: (f32, f32),
    pub color: Color,
    pub repr: char,
}

impl Ball {
    const REPRS: &'static str = "●◉❖▲✢✦★❤";

    pub fn new(board: &Board) -> Ball {
        let mut rng = rand::thread_rng();
        let r = |i: i32| rand::random::<f32>() * i as f32; // WOW, I couldn't find a way to use `rng` here.
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

    pub fn pos_i32(&self) -> (i32, i32) {
        (self.position.0 as i32, self.position.1 as i32)
    }
}

impl Display for Ball {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        style!(f, self.color, self.repr)?;
        write!(f, "\tposition{:?}", self.position)?;
        write!(f, "\tvelocity{:?}", self.velocity)
    }
}
