use rand::distributions::{Distribution, Standard};
use rand::Rng;

macro_rules! style {
    ($f:ident, $s:expr, $t:expr) => {
        write!($f, "{}{}{}", $s.data(), $t, $crate::colors::Style::RESET)
    };
}

// I could use something like this to avoid having to manually add
// the same constants in enum, data, and sample...

// macro_rules! styled_enum {
//     ($($name:ident => $aec:expr),*) => {
//     pub enum $name
//     }
// }

#[allow(dead_code)]
#[derive(Debug)]
pub enum Color {
    BLUE,
    GREEN,
    YELLOW,
    RED,
    MAGENTA,
    CYAN,
    ORANGE,
}

impl Color {
    pub fn data(&self) -> &str {
        match *self {
            Color::BLUE => "\x1b[94m",
            Color::GREEN => "\x1b[92m",
            Color::YELLOW => "\x1b[93m",
            Color::RED => "\x1b[91m",
            Color::MAGENTA => "\x1b[95m",
            Color::CYAN => "\x1b[96m",
            Color::ORANGE => "\x1b[38;5;208m",
        }
    }
}

impl Distribution<Color> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Color {
        match rng.gen_range(0..7) {
            0 => Color::BLUE,
            1 => Color::GREEN,
            2 => Color::YELLOW,
            3 => Color::RED,
            4 => Color::MAGENTA,
            5 => Color::CYAN,
            _ => Color::ORANGE,
        }
    }
}

#[allow(dead_code)]
#[derive(Debug)]
pub enum Style {
    BOLD,
    DIM,
    ITALIC,
    UNDERLINE,
}

impl Style {
    pub const RESET: &'static str = "\x1b[0m";

    pub fn data(&self) -> &str {
        match *self {
            Style::BOLD => "\x1b[1m",
            Style::DIM => "\x1b[2m",
            Style::ITALIC => "\x1b[3m",
            Style::UNDERLINE => "\x1b[4m",
        }
    }
}
