use crate::colors::Style;
use crate::game::{Component, FrameBuffer, World};
use crate::utils::{Point, Size};

#[derive(Debug)]
pub struct Board {
    pub size: Size,
}

impl Board {
    pub fn new(size: Size) -> Board {
        Board { size }
    }
}

impl Component for Board {
    fn update(&mut self, world: &World) {}

    fn draw_to(&self, frame_buffer: &mut FrameBuffer) {
        let (border_x, border_y) = (self.size.w - 1, self.size.h - 1);
        (0..self.size.w).for_each(|x| {
            frame_buffer.draw(Point { x, y: 0 }, Style::ORANGE, '█');
            frame_buffer.draw(Point { x, y: border_y }, Style::ORANGE, '█');
        });
        (1..self.size.h - 1).for_each(|y| {
            frame_buffer.draw(Point { x: 0, y }, Style::ORANGE, '█');
            frame_buffer.draw(Point { x: border_x, y }, Style::ORANGE, '█');
        });
    }

    fn size_hint(&self) -> u16 {
        self.size.w * 2 + self.size.h * 2
    }
}
