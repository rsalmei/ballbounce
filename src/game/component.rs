use crate::game::world::World;
use crate::game::FrameBuffer;

pub trait Component {
    fn update(&mut self, board: &World);
    fn draw_to(&self, frame_buffer: &mut FrameBuffer);
    fn size_hint(&self) -> u16;
}
