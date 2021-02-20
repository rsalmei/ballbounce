#[derive(Debug)]
pub struct Board {
    pub size: (usize, usize),
}

impl Board {
    pub fn new() -> Board {
        let size = (81, 20);
        Board { size }
    }
}
