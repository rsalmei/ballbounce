#[derive(Debug)]
pub struct Size(pub usize, pub usize);

#[derive(Debug)]
pub struct Velocity(pub f32, pub f32);

#[derive(Debug)]
pub struct Point<T>(pub T, pub T);

impl Point<f32> {
    pub fn truncate(&self) -> Point<usize> {
        Point(self.0 as usize, self.1 as usize)
    }
}
