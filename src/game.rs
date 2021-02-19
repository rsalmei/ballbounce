use crate::ball::Ball;
use crate::board::Board;
use crate::colors::Style;

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

    pub fn tick(&mut self) {
        for b in self.balls.iter_mut() {
            b.update(&self.board)
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
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
                if let Some(ball) = self.balls.iter().find(|b| b.pos_i32() == (c, r)) {
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
