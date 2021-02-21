use crate::ball::Ball;
use crate::board::Board;
use crate::colors::Style;
use std::fmt::{self, Display, Formatter};

#[derive(Debug)]
pub struct Game {
    pub board: Board,
    pub balls: Vec<Ball>,
}

impl Game {
    pub fn new(num_balls: usize) -> Game {
        let board = Board::new();
        let balls = (0..num_balls).map(|_| Ball::new(&board)).collect();
        Game { board, balls }
    }

    pub fn process_input(&mut self) {}

    pub fn update(&mut self) {
        for b in self.balls.iter_mut() {
            b.update(&self.board)
        }
    }

    pub fn render(&mut self) {
        for b in self.balls.iter() {
            b.draw_to(&mut self.frame_buffer_back)
        }
        print!("{}", self);
    }

    pub fn caption(&self) -> String {
        self.balls
            .iter()
            .enumerate()
            .map(|(i, b)| format!("{}: {}", i + 1, b))
            .collect::<Vec<_>>()
            .join("\n")
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        let border = |f: &mut Formatter<'_>| {
            write!(f, "{}+", Style::RED)?;
            for _ in 0..self.board.size.0 {
                write!(f, "-")?;
            }
            write!(f, "+{}", Style::RESET)
        };

        border(f)?;
        for r in 0..self.board.size.1 {
            write!(f, "\n{}", style!(Style::RED, "|"))?;
            for c in 0..self.board.size.0 {
                if let Some(ball) = self.balls.iter().find(|b| b.actual_pos() == (c, r)) {
                    write!(f, "{}", style!(ball.color, ball.repr))?
                } else {
                    write!(f, " ")?
                }
            }
            write!(f, "{}", style!(Style::RED, "|"))?
        }
        writeln!(f)?;
        border(f)
    }
}
