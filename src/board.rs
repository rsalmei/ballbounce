use crate::game::FrameBuffer;
use crate::utils::Size;

#[derive(Debug)]
pub struct Board {
    pub size: Size,
}

impl Board {
    pub fn new(size: Size) -> Board {
        Board { size }
    }

    pub fn draw_to(&self, _frame_buffer: &mut FrameBuffer) {}
}
