use crate::colors::Style;
use crate::game::FrameBuffer;
use crate::utils::{Point, Size};

#[derive(Debug)]
pub struct Board {
    pub size: Size,
}

impl Board {
    pub fn new(size: Size) -> Board {
        Board { size }
    }

    pub fn draw_to(&self, frame_buffer: &mut FrameBuffer) {
        let (border_x, border_y) = (self.size.w - 1, self.size.h - 1);
        // FIXME: WHY the render time skyrockets if I uncomment any of these lines?? The lines below seems almost identical and are fine...
        for x in 0..self.size.w {
            // frame_buffer.draw(Point { x, y: 0 }, Style::ORANGE, '█');
            // frame_buffer.draw(Point { x, y: border_y }, Style::ORANGE, '█');
        }
        for y in 0..self.size.h {
            frame_buffer.draw(Point { x: 0, y }, Style::ORANGE, '█');
            frame_buffer.draw(Point { x: border_x, y }, Style::ORANGE, '█');
        }
    }
}
