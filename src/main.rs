mod colors;

use colors::Color;
use rand::seq::IteratorRandom;
use rand::Rng;
use std::fmt::{Display, Formatter, Result};
use std::thread;
use std::time::Duration;

#[derive(Debug)]
struct Game {
    board: Board,
    balls: Vec<Ball>,
}

#[derive(Debug)]
struct Board {
    size: (i32, i32),
}

#[derive(Debug)]
struct Ball {
    position: (f32, f32),
    velocity: (f32, f32),
    color: Color,
    repr: char,
}

impl Game {
    fn new(num_balls: usize) -> Game {
        let board = Board::new();
        let balls = (0..num_balls).map(|_| Ball::new(&board)).collect();
        Game { board, balls }
    }

    fn tick(&mut self) {
        for b in self.balls.iter_mut() {
            b.update(&self.board)
        }
    }
}

impl Board {
    fn new() -> Board {
        Board { size: (81, 20) }
    }
}

impl Ball {
    const REPRS: &'static str = "●◉❖▲✢✦★❤";

    fn new(board: &Board) -> Ball {
        let mut rng = rand::thread_rng();
        let r = |i: i32| rand::random::<f32>() * i as f32; // WOW, I couldn't find a way to use `rng` here.
        let v = || r(4) - 2.;
        Ball {
            position: (r(board.size.0), r(board.size.1)),
            velocity: (v(), v()),
            color: rng.gen(),
            repr: Ball::REPRS.chars().choose(&mut rng).unwrap(),
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

    fn pos_i32(&self) -> (i32, i32) {
        (self.position.0 as i32, self.position.1 as i32)
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

        border(f)?;
        for r in 0..self.board.size.1 {
            write!(f, "\n|")?;
            for c in 0..self.board.size.0 {
                if let Some(ball) = self.balls.iter().find(|b| b.pos_i32() == (c, r)) {
                    style!(f, ball.color, ball.repr)?
                } else {
                    write!(f, " ")?
                }
            }
            write!(f, "|")?
        }
        writeln!(f)?;
        border(f)
    }
}

impl Display for Ball {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        style!(f, self.color, self.repr)?;
        write!(f, "\tposition{:?}", self.position)?;
        write!(f, "\tvelocity{:?}", self.velocity)
    }
}

fn main() {
    let mut game = Game::new(5);
    println!(
        "{}",
        game.balls
            .iter()
            .enumerate()
            .map(|(i, b)| format!("{}: {}", i + 1, b))
            .collect::<Vec<_>>()
            .join("\n")
    );
    let delay = Duration::from_secs_f32(1. / 30.);
    loop {
        print!("{}", game);
        thread::sleep(delay);
        game.tick();
        println!("\x1b[{}A", game.board.size.1 + 2);
    }
}
