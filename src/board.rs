use crate::colors::Style;
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
            frame_buffer.draw(Point { x, y: 0 }, Style::Orange, '█');
            frame_buffer.draw(Point { x, y: border_y }, Style::Orange, '█');
        });
        (1..self.size.h - 1).for_each(|y| {
            frame_buffer.draw(Point { x: 0, y }, Style::Orange, '█');
            frame_buffer.draw(Point { x: border_x, y }, Style::Orange, '█');
        });
    }

    fn size_hint(&self) -> u16 {
        self.size.w * 2 + self.size.h * 2
    }
}
