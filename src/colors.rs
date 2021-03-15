use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::{self, Display, Formatter};

/// This enables to style only parts of a text, inside the same format! or write!.
/// Example:
///     write!(f, "result: {}, time: {}", style!(Style::RED, result), time)?;
macro_rules! style {
    ($s:expr, $t:expr) => {
        format_args!("{}{}{}", $s.data(), $t, $crate::colors::Style::RESET)
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
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Ord, PartialOrd)]
pub enum Style {
    // colors.
    BLUE,
    GREEN,
    YELLOW,
    RED,
    MAGENTA,
    CYAN,
    ORANGE,
    // decorations.
    BOLD,
    DIM,
    ITALIC,
    UNDERLINE,
    RESET,
}

impl Display for Style {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.data())
    }
}

impl Style {
    pub const NUM_COLORS: usize = 7;

    pub fn data(&self) -> &str {
        match self {
            Style::BLUE => "\x1b[94m",
            Style::GREEN => "\x1b[92m",
            Style::YELLOW => "\x1b[93m",
            Style::RED => "\x1b[91m",
            Style::MAGENTA => "\x1b[95m",
            Style::CYAN => "\x1b[96m",
            Style::ORANGE => "\x1b[38;5;208m",
            Style::BOLD => "\x1b[1m",
            Style::DIM => "\x1b[2m",
            Style::ITALIC => "\x1b[3m",
            Style::UNDERLINE => "\x1b[4m",
            Style::RESET => "\x1b[0m",
        }
    }
}

impl Distribution<Style> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Style {
        match rng.gen_range(0..Style::NUM_COLORS) {
            0 => Style::BLUE,
            1 => Style::GREEN,
            2 => Style::YELLOW,
            3 => Style::RED,
            4 => Style::MAGENTA,
            5 => Style::CYAN,
            _ => Style::ORANGE,
        }
    }
}
