use crate::game::world::World;
use crate::game::FrameBuffer;
use termion::event::Key;

pub trait Component {
    fn action(&mut self, _key: Key) {}
    fn update(&mut self, _board: &World) {}
    fn draw_to(&self, frame_buffer: &mut FrameBuffer);
    fn size_hint(&self) -> u16 {
        1
    }
}
