use crate::colors::Style;
use std::fmt::{self, Formatter};

#[derive(Clone, Debug)]
pub struct FrameBuffer(Vec<Vec<Option<(Style, char)>>>);

impl FrameBuffer {
    pub(super) fn new(size: (usize, usize)) -> FrameBuffer {
        FrameBuffer((0..size.1).map(|_| vec![None; size.0]).collect())
    }

    pub(super) fn clear(&mut self) {
        for r in &mut self.0 {
            for c in r {
                *c = None;
            }
        }
    }

    pub fn draw(&mut self, pos: (usize, usize), style: Style, repr: char) {
        *&mut self.0[pos.1][pos.0] = Some((style, repr));
    }

    pub(super) fn fmt_row(&self, f: &mut Formatter<'_>, row: usize) -> fmt::Result {
        for p in &self.0[row] {
            if let Some((s, c)) = p {
                write!(f, "{}", style!(s, c))?;
            } else {
                write!(f, " ")?;
            };
        }
        Ok(())
    }
}
