use crate::domain::Size;

#[derive(Debug)]
pub struct World {
    pub size: Size,
}

impl World {
    pub fn new(size: Size) -> World {
        World { size }
    }
}
