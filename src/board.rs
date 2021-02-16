#[derive(Debug)]
pub struct Board {
    pub size: (i32, i32),
}

impl Board {
    pub fn new() -> Board {
        Board { size: (81, 20) }
    }
}
