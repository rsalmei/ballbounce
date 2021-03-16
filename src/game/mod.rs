mod component;
mod frame_buffer;
mod world;

use crate::ball::{Ball, BallBuilder};
use crate::board::Board;
use crate::colors::Style;
use crate::utils::Size;
pub use component::Component;
pub use frame_buffer::FrameBuffer;
use std::io::{self, Write};
use termion::event::Key;
pub use world::World;

pub struct Game {
    world: World,
    board: Board,
    balls: Vec<Ball>,
    frame_buffer: FrameBuffer,
}

impl Game {
    pub fn new(size: Size, mut num_balls: usize) -> Game {
        let world = World::new(size);
        let board = Board::new(size);

        num_balls = num_balls.max(1).min(Ball::COMBINATIONS);
        let mut balls = Vec::with_capacity(num_balls + 1);
        balls.push(
            BallBuilder::new()
                .with_color(Style::RED)
                .with_repr('◉')
                .build(&world),
        );
        while balls.len() < num_balls {
            balls.push(loop {
                let candidate = BallBuilder::new().build(&world);
                if balls.iter().all(|b| &candidate != b) {
                    break candidate;
                }
            });
        }

        let capacity = board.size_hint() + balls.iter().map(Component::size_hint).sum::<u16>();
        let frame_buffer = FrameBuffer::new(capacity);
        Game {
            world,
            board,
            balls,
            frame_buffer,
        }
    }

    pub fn process_input(&mut self, key: Key) {
    }

    pub fn update(&mut self) {
        self.board.update(&self.world);
        for c in self.balls.iter_mut() {
            c.update(&self.world)
        }
    }

    pub fn render(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        self.board.draw_to(&mut self.frame_buffer);
        for c in self.balls.iter() {
            c.draw_to(&mut self.frame_buffer)
        }
        self.frame_buffer.render_to(stdout)
    }

    pub fn num_balls(&self) -> usize {
        self.balls.len()
    }
}
