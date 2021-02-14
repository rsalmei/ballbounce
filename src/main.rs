use std::fmt::{Display, Formatter, Result};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Game {
    board: Board,
    ball: Ball,
}

#[derive(Debug)]
struct Board {
    size: (i32, i32),
}

#[derive(Debug)]
struct Ball {
    pos: (f32, f32),
    velocity: (f32, f32),
}

impl Game {
    fn new() -> Game {
        let board = Board::new();
        let ball = Ball::new(&board);
        Game { board, ball }
    }

    fn tick(&mut self) {
        self.ball.update(&self.board);
    }
}

impl Board {
    fn new() -> Board {
        Board { size: (81, 20) }
    }
}

impl Ball {
    fn new(board: &Board) -> Ball {
        let r = |i: i32| rand::random::<f32>() * i as f32;
        let v = || r(6) - 3.;
        Ball {
            pos: (r(board.size.0), r(board.size.1)),
            velocity: (v(), v()),
        }
    }

    fn update(&mut self, board: &Board) {
        self.pos = (self.pos.0 + self.velocity.0, self.pos.1 + self.velocity.1);
        if self.pos.0 < 0. || self.pos.0 >= board.size.0 as f32 {
            self.velocity.0 = -self.velocity.0;
            self.pos.0 += self.velocity.0
        }
        if self.pos.1 < 0. || self.pos.1 >= board.size.1 as f32 {
            self.velocity.1 = -self.velocity.1;
            self.pos.1 += self.velocity.1
        }
    }
}

impl Display for Game {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        let border = |f: &mut Formatter<'_>| {
            write!(f, "+")?;
            for _ in 0..self.board.size.0 {
                write!(f, "-")?;
            }
            write!(f, "+")?;
            Ok(())
        };
        border(f)?;
        for r in 0..self.board.size.1 {
            write!(f, "\n|")?;
            if r == self.ball.pos.1 as i32 {
                for c in 0..self.board.size.0 {
                    write!(
                        f,
                        "{}",
                        if c == self.ball.pos.0 as i32 {
                            "\x1b[91m◉\x1b[0m"
                        } else {
                            " "
                        }
                    )?
                }
            } else {
                for _ in 0..self.board.size.0 {
                    write!(f, " ")?
                }
            }
            write!(f, "|")?
        }
        write!(f, "\n")?;
        border(f)
    }
}

fn main() {
    let mut game = Game::new();
    println!("{:?}", game.ball);
    let delay = Duration::from_secs_f32(1. / 30.);
    loop {
        println!("{}", game);
        thread::sleep(delay);
        game.tick();
        println!("\x1b[{}A", game.board.size.1 + 3);
    }
}
