use crate::color;
use crate::component::Component;
use crate::domain::{Point, Size};
use crate::frame_buffer::FrameBuffer;

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
    fn draw_to(&self, frame_buffer: &mut FrameBuffer) {
        let (border_x, border_y) = (self.size.w - 1, self.size.h - 1);
        (0..self.size.w).for_each(|x| {
            frame_buffer.draw(Point { x, y: 0 }, color::ORANGE, '█');
            frame_buffer.draw(Point { x, y: border_y }, color::ORANGE, '█');
        });
        (1..self.size.h - 1).for_each(|y| {
            frame_buffer.draw(Point { x: 0, y }, color::ORANGE, '█');
            frame_buffer.draw(Point { x: border_x, y }, color::ORANGE, '█');
        });
    }

    fn size_hint(&self) -> usize {
        (self.size.w as usize + self.size.h as usize) * 2
    }
}
