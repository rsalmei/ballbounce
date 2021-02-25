use crate::colors::Style;
use itertools::Itertools;
use std::fmt::{self, Display, Formatter};

#[derive(Clone, Debug)]
pub struct FrameBuffer(Vec<FrameRow>);

#[derive(Clone, Debug)]
pub struct FrameRow(Vec<Option<(Style, char)>>);

impl FrameBuffer {
    pub(super) fn new(size: (usize, usize)) -> FrameBuffer {
        FrameBuffer((0..size.1).map(|_| FrameRow(vec![None; size.0])).collect())
    }

    pub(super) fn clear(&mut self) {
        for r in &mut self.0 {
            for c in &mut r.0 {
                *c = None;
            }
        }
    }

    pub fn draw(&mut self, pos: (usize, usize), style: Style, repr: char) {
        *&mut self.0[pos.1].0[pos.0] = Some((style, repr));
    }


impl Display for FrameRow {
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
