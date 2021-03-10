use crate::board::Board;
use crate::colors::Style;
use crate::game::FrameBuffer;
use crate::utils::{Point, Velocity};
use rand::rngs::ThreadRng;
use rand::seq::SliceRandom;
use rand::Rng;
use std::fmt::{Display, Formatter, Result};

#[derive(Debug)]
pub struct Ball {
    position: Point<f32>,
    velocity: Velocity,
    pub color: Style,
    pub repr: char,
}

#[derive(Default)]
pub struct BallBuilder {
    color: Option<Style>,
    repr: Option<char>,
}

impl BallBuilder {
    pub fn new() -> BallBuilder {
        BallBuilder::default()
    }

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
    const REPRS: [char; 12] = ['◉', '●', '❖', '▲', '✢', '✦', '★', '☻', '❤', '♠', '♣', '♦'];
    pub const COMBINATIONS: usize = Ball::REPRS.len() * Style::NUM_COLORS;

    fn build(board: &Board, color: Option<Style>, repr: Option<char>) -> Ball {
        let mut rng = rand::thread_rng();
        let r = |i: usize, rng: &mut ThreadRng| rng.gen::<f32>() * i as f32;
        let v = |rng: &mut ThreadRng| r(4, rng) - 2.;
        Ball {
            position: Point {
                x: r(board.size.0, &mut rng),
                y: r(board.size.1, &mut rng),
            },
            velocity: Velocity {
                vx: v(&mut rng),
                vy: v(&mut rng),
            },
            color: color.unwrap_or_else(|| rng.gen()),
            repr: repr.unwrap_or_else(|| *Ball::REPRS.choose(&mut rng).unwrap()),
        }
    }

    pub fn update(&mut self, board: &Board) {
        self.position = Point {
            x: self.position.x + self.velocity.vx,
            y: self.position.y + self.velocity.vy,
        };
        if self.position.x < 0. || self.position.x >= board.size.0 as f32 {
            self.velocity.vx = -self.velocity.vx;
            self.position.x += self.velocity.vx
        }
        if self.position.y < 0. || self.position.y >= board.size.1 as f32 {
            self.velocity.vy = -self.velocity.vy;
            self.position.y += self.velocity.vy
        }
    }

    pub fn draw_to(&self, frame_buffer: &mut FrameBuffer) {
        frame_buffer.draw(self.position.truncate(), self.color, self.repr);
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
