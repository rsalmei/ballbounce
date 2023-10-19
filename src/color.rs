#![allow(dead_code)]

use rand::distributions::{Distribution, Standard};
use rand::Rng;

/// This enables to style only parts of a text, inside the same format! or write!.
/// Example:
///     write!(f, "result: {}, time: {}", style!(Style::RED, result), time)?;
macro_rules! style {
    ($s:expr, $t:expr) => {
        format_args!("{}{}\x1b[0m", $s.code(), $t)
    };
}

// I could use something like this to avoid having to manually add
// the same constants in enum, data, and sample...

// macro_rules! styled_enum {
//     ($($name:ident => $aec:expr),*) => {
//     pub enum $name
//     }
// }

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Style(&'static str);

impl Style {
    pub fn code(&self) -> &'static str {
        self.0
    }
}

pub const NUM_COLORS: usize = 7;

// colors.
pub const BLUE: Style = Style("\x1b[94m");
pub const GREEN: Style = Style("\x1b[92m");
pub const YELLOW: Style = Style("\x1b[93m");
pub const RED: Style = Style("\x1b[91m");
pub const MAGENTA: Style = Style("\x1b[95m");
pub const CYAN: Style = Style("\x1b[96m");
pub const ORANGE: Style = Style("\x1b[38;5;208m");
// decorations.
pub const BOLD: Style = Style("\x1b[1m");
pub const DIM: Style = Style("\x1b[2m");
pub const ITALIC: Style = Style("\x1b[3m");
pub const UNDERLINE: Style = Style("\x1b[4m");

impl Distribution<Style> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Style {
        match rng.gen_range(0..NUM_COLORS) {
            0 => BLUE,
            1 => GREEN,
            2 => YELLOW,
            3 => RED,
            4 => MAGENTA,
            5 => CYAN,
            _ => ORANGE,
        }
    }
}
