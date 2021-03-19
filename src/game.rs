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
    pub fn new(size: Size, num_balls: usize) -> Game {
        let world = World::new(size);
        let board = Board::new(size);

        let mut balls = vec![BallBuilder::new()
            .with_color(Style::RED)
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

    pub fn process_input(&mut self, key: Key) {
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
