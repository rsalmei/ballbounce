use crate::colors::Style;
use crate::utils::Point;
use std::collections::{HashMap, HashSet};
use std::io::{self, Write};
use std::mem;
use termion::cursor;

#[derive(Debug)]
pub struct FrameBuffer {
    data: HashMap<Point<u16>, (Style, char)>,
    data_back: HashMap<Point<u16>, (Style, char)>,
}

impl FrameBuffer {
    pub(super) fn new(capacity: u16) -> FrameBuffer {
        let capacity = capacity as usize;
        FrameBuffer {
            data: HashMap::with_capacity(capacity),
            data_back: HashMap::with_capacity(capacity),
        }
    }

    pub fn draw(&mut self, pos: Point<u16>, style: Style, repr: char) {
        self.data_back.insert(pos, (style, repr));
    }

    pub(super) fn render_to(&mut self, stdout: &mut impl Write) -> io::Result<()> {
        mem::swap(&mut self.data, &mut self.data_back);

        let old = self.data_back.keys().collect::<HashSet<_>>();
        let new = self.data.keys().collect::<HashSet<_>>();
        display_clear(stdout, old.difference(&new))?;

        let old = self.data_back.iter().collect::<HashSet<_>>();
        let new = self.data.iter().collect::<HashSet<_>>();
        display_buffer(stdout, new.difference(&old))?;

        self.data_back.clear();
        stdout.flush()
    }
}

fn display_clear<'a>(
    stdout: &mut impl Write,
    data: impl Iterator<Item = &'a &'a Point<u16>>,
) -> io::Result<()> {
    for point in data {
        write!(stdout, "{} ", cursor::Goto(point.x + 1, point.y + 1))?
    }
    Ok(())
}

fn display_buffer<'a>(
    stdout: &mut impl Write,
    data: impl Iterator<Item = &'a (&'a Point<u16>, &'a (Style, char))>,
) -> io::Result<()> {
    for (point, (style, repr)) in data {
        write!(
            stdout,
            "{}{}",
            cursor::Goto(point.x + 1, point.y + 1),
            style!(style, repr)
        )?
    }
    Ok(())
}
