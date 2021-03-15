#[derive(Copy, Clone, Debug)]
pub struct Size {
    pub w: u16,
    pub h: u16,
}

#[derive(Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Clone, Debug, PartialEq, Hash, Ord, PartialOrd)]
pub struct Point<T> {
    pub y: T,
    pub x: T,
}

impl Point<f32> {
    pub fn truncate(&self) -> Point<u16> {
        Point {
            x: self.x as u16,
            y: self.y as u16,
        }
    }
}

impl<T: Eq> Eq for Point<T> {}
