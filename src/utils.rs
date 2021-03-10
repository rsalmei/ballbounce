#[derive(Debug)]
pub struct Size(pub usize, pub usize);

#[derive(Debug)]
pub struct Velocity {
    pub vx: f32,
    pub vy: f32,
}

#[derive(Debug)]
pub struct Point<T> {
    pub x: T,
    pub y: T,
}

impl Point<f32> {
    pub fn truncate(&self) -> Point<usize> {
        Point {
            x: self.x as usize,
            y: self.y as usize,
        }
    }
}
