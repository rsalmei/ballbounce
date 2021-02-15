use std::fmt::{Display, Formatter, Result};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Game {
    frame: usize,
    board: Board,
    balls: Vec<Ball>,
    balls_pos: Vec<(i32, i32)>,
}

#[derive(Debug)]
struct Board {
    size: (i32, i32),
}

#[derive(Debug)]
struct Ball {
    position: (f32, f32),
    velocity: (f32, f32),
}

impl Game {
    fn new(num_balls: usize) -> Game {
        let board = Board::new();
        let balls = (0..num_balls).map(|_| Ball::new(&board)).collect();
        Game {
            frame: 0,
            board,
            balls,
            balls_pos: vec![(0, 0); num_balls],
        }
    }

    fn tick(&mut self) {
        for b in self.balls.iter_mut() {
            b.update(&self.board)
        }
        self.balls_pos
            .iter_mut()
            .zip(self.balls.iter())
            .for_each(|(p, b)| *p = (b.pos.0 as i32, b.pos.1 as i32));
        self.frame += 1;
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
            position: (r(board.size.0), r(board.size.1)),
            velocity: (v(), v()),
        }
    }

    fn update(&mut self, board: &Board) {
        self.position = (
            self.position.0 + self.velocity.0,
            self.position.1 + self.velocity.1,
        );
        if self.position.0 < 0. || self.position.0 >= board.size.0 as f32 {
            self.velocity.0 = -self.velocity.0;
            self.position.0 += self.velocity.0
        }
        if self.position.1 < 0. || self.position.1 >= board.size.1 as f32 {
            self.velocity.1 = -self.velocity.1;
            self.position.1 += self.velocity.1
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
            write!(f, "+")
        };

        writeln!(f, "frame: {}", self.frame)?;
        border(f)?;
        for r in 0..self.board.size.1 {
            write!(f, "\n|")?;
            for c in 0..self.board.size.0 {
                write!(
                    f,
                    "{}",
                    if self.balls_pos.iter().any(|&(x, y)| x == c && y == r) {
                        "\x1b[91m◉\x1b[0m"
                    } else {
                        " "
                    }
                )?
            }
            write!(f, "|")?
        }
        writeln!(f)?;
        border(f)
    }
}

fn main() {
    let mut game = Game::new(5);
    println!(
        "{}",
        game.balls
            .iter()
            .enumerate()
            .map(|(i, b)| format!("{}: {:?}", i + 1, b))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let delay = Duration::from_secs_f32(1. / 30.);
    loop {
        print!("{}", game);
        thread::sleep(delay);
        game.tick();
        println!("\x1b[{}A", game.board.size.1 + 3);
    }
}
