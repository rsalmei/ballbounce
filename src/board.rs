use crate::game::FrameBuffer;

#[derive(Debug)]
pub struct Board {
    pub size: (usize, usize),
}

impl Board {
    pub fn new() -> Board {
        let size = (100, 30);
        Board { size }
    }

    pub fn draw_to(&self, _frame_buffer: &mut FrameBuffer) {}
}
