use crate::board::Board;
use crate::frame_buffer::FrameBuffer;
use termion::event::Key;

pub trait Component {
    fn action(&mut self, _key: Key) {}
    fn update(&mut self, _board: &Board) {}
    fn draw_to(&self, frame_buffer: &mut FrameBuffer);
    fn size_hint(&self) -> u16 {
        1
    }
}
