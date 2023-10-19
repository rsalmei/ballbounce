use crate::board::Board;
use crate::color::{self, Style};
use crate::component::Component;
use crate::domain::{Point, Velocity};
use crate::frame_buffer::FrameBuffer;
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Ball {
    pub position: Point<f32>,
    pub velocity: Velocity,
    pub color: Style,
    pub repr: char,
}

#[derive(Default)]
pub struct BallBuilder {
    position: Option<Point<f32>>,
    velocity: Option<Velocity>,
    color: Option<Style>,
    repr: Option<char>,
}

impl BallBuilder {
    pub fn new() -> BallBuilder {
        BallBuilder::default()
    }

    pub fn with_position(&mut self, position: Point<f32>) -> &mut BallBuilder {
        self.position = Some(position);
        self
    }

    pub fn with_velocity(&mut self, velocity: Velocity) -> &mut BallBuilder {
        self.velocity = Some(velocity);
        self
    }

    pub fn with_color(&mut self, color: Style) -> &mut BallBuilder {
        self.color = Some(color);
        self
    }

    pub fn with_repr(&mut self, repr: char) -> &mut BallBuilder {
        self.repr = Some(repr);
        self
    }

    pub fn build_one(&self, board: &Board) -> Ball {
        let mut rng = rand::thread_rng();
        let r = |i: u16, rng: &mut ThreadRng| rng.gen::<f32>() * i as f32;
        let v = |rng: &mut ThreadRng| r(4, rng) - 2.;
        Ball {
            position: self.position.unwrap_or_else(|| Point {
                x: r(board.size.w, &mut rng),
                y: r(board.size.h, &mut rng),
            }),
            velocity: self.velocity.unwrap_or_else(|| Velocity {
                vx: v(&mut rng),
                vy: v(&mut rng),
            }),
            color: self.color.unwrap_or_else(|| rng.gen()),
            repr: self
                .repr
                .unwrap_or_else(|| *Ball::REPRS.choose(&mut rng).unwrap()),
        }
    }

    pub fn build_multiple(&self, mut num_balls: usize, target: &mut Vec<Ball>, board: &Board) {
        const RETRIES: usize = 10; // try to minimize repetitions.
        target.reserve(num_balls);
        while num_balls > 0 {
            target.push(
                (0..RETRIES)
                    .map(|_| self.build_one(board))
                    .find(|candidate| target.iter().all(|b| candidate != b))
                    .unwrap_or_else(|| self.build_one(board)),
            );
            num_balls -= 1;
        }
    }
}

impl Ball {
    const REPRS: &[char] = &['◉', '●', '❖', '▲', '✢', '✦', '★', '☻', '❤', '♠', '♣', '♦'];
}

impl Component for Ball {
    fn update(&mut self, board: &Board) {
        self.position = Point {
            x: self.position.x + self.velocity.vx,
            y: self.position.y + self.velocity.vy,
        };
        if self.position.x < 0. || self.position.x >= board.size.w as f32 {
            self.velocity.vx = -self.velocity.vx;
            self.position.x += self.velocity.vx
        }
        if self.position.y < 0. || self.position.y >= board.size.h as f32 {
            self.velocity.vy = -self.velocity.vy;
            self.position.y += self.velocity.vy
        }
    }

    fn draw_to(&self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.draw(self.position.truncate(), self.color, self.repr);
    }
}

impl Display for Ball {
    // unused for now.
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", style!(self.color, self.repr))?;
        write!(f, "{}{:?}", style!(color::DIM, "\tposition"), self.position)?;
        write!(f, "{}{:?}", style!(color::DIM, "\tvelocity"), self.velocity)
    }
}

impl PartialEq for Ball {
    fn eq(&self, other: &Self) -> bool {
        self.color == other.color && self.repr == other.repr
    }
}
