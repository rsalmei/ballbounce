use crate::colors::Style;
use crate::utils::{Point, Size};
use itertools::Itertools;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct FrameBuffer {
    cols: usize,
    data: Vec<Option<(Style, char)>>,
}

#[derive(Clone, Debug)]
pub struct FrameRow<'a>(&'a [Option<(Style, char)>]);

impl FrameBuffer {
    pub(super) fn new(size: &Size) -> FrameBuffer {
        FrameBuffer {
            cols: size.0,
            data: vec![None; size.0 * size.1],
        }
    }

    pub(super) fn clear(&mut self) {
        for c in &mut self.data {
            *c = None;
        }
    }

    pub fn draw(&mut self, pos: Point<usize>, style: Style, repr: char) {
        self.data[pos.y * self.cols + pos.x] = Some((style, repr));
    }

    pub fn iter(&self) -> impl Iterator<Item = FrameRow> + '_ {
        self.data.chunks(self.cols).map(|row| FrameRow(row))
    }
}

impl<'a> Display for FrameRow<'a> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            self.0.iter().format_with("", |p, f| {
                match p {
                    None => f(&" "),
                    Some((s, c)) => f(&style!(s, c)),
                }
            })
        )
    }
}
