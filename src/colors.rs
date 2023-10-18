use rand::distributions::{Distribution, Standard};
use rand::Rng;
use std::fmt::{self, Display, Formatter};

/// This enables to style only parts of a text, inside the same format! or write!.
/// Example:
///     write!(f, "result: {}, time: {}", style!(Style::RED, result), time)?;
macro_rules! style {
    ($s:expr, $t:expr) => {
        format_args!("{}{}{}", $s.data(), $t, $crate::colors::Style::Reset)
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
    Blue,
    Green,
    Yellow,
    Red,
    Magenta,
    Cyan,
    Orange,
    // decorations.
    Bold,
    Dim,
    Italic,
    Underline,
    Reset,
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
            Style::Blue => "\x1b[94m",
            Style::Green => "\x1b[92m",
            Style::Yellow => "\x1b[93m",
            Style::Red => "\x1b[91m",
            Style::Magenta => "\x1b[95m",
            Style::Cyan => "\x1b[96m",
            Style::Orange => "\x1b[38;5;208m",
            Style::Bold => "\x1b[1m",
            Style::Dim => "\x1b[2m",
            Style::Italic => "\x1b[3m",
            Style::Underline => "\x1b[4m",
            Style::Reset => "\x1b[0m",
        }
    }
}

impl Distribution<Style> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Style {
        match rng.gen_range(0..Style::NUM_COLORS) {
            0 => Style::Blue,
            1 => Style::Green,
            2 => Style::Yellow,
            3 => Style::Red,
            4 => Style::Magenta,
            5 => Style::Cyan,
            _ => Style::Orange,
        }
    }
}
