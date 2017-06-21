use termion::cursor::{Goto, Right, Down};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, terminal_size};
use std::io::{Write, Stdout, stdout};
use std::fmt::Display;
use std::cmp::{max, min};
use ::errors::Result;

const MIN_HEIGHT: u16 = 5;

use ::util;

pub struct Dropdown {
    stdout: RawTerminal<Stdout>,
    start: Goto,
    origin: Goto,
    pub max_height: u16,
    pub height: u16,
}

impl Dropdown {
    pub fn new(max_height: u16) -> Self {
        let mut out = stdout().into_raw_mode().unwrap();
        let (x, y) = util::sync_cursor_pos(&mut out).unwrap();
        let origin = if x == 1 { Goto(1, y) } else { Goto(1, y+1) };
        Self {
            start: Goto(x, y),
            stdout: out,
            height: max_height,
            max_height,
            origin
        }
    }

    pub fn goto_origin(&mut self) -> Result<&mut Self> {
        write!(self.stdout, "{}", self.origin)?;
        Ok(self)
    }

    pub fn resize(&mut self) -> Result<&mut Self> {
        self.height = max(MIN_HEIGHT, min(util::window_height()? - self.start.1, self.max_height));
        Ok(self)
    }

    pub fn reset(&mut self) -> Result<&mut Self> {
        self.resize()?.goto_origin()?;
        for _ in 0..(self.height+1) {
            self.write(format!("{}\n", clear::CurrentLine))?;
        }
        self.origin.1 = min(self.origin.1, terminal_size().unwrap().1 - self.height);
        self.goto_origin()?;
        Ok(self)
    }

    pub fn clearline(&mut self) -> Result<&mut Self> {
        write!(self.stdout, "{}\r", clear::CurrentLine)?;
        Ok(self)
    }

    pub fn write<D: Display>(&mut self, value: D) -> Result<&mut Self> {
        write!(self.stdout, "{}", value)?;
        Ok(self)
    }

    pub fn writeln<D: Display>(&mut self, value: D) -> Result<&mut Self> {
        self.write(Down(1))?.clearline()?.write(value)
    }

    pub fn flush(&mut self) -> Result<&mut Self> {
        self.stdout.flush()?;
        Ok(self)
    }

    pub fn set_cursor(&mut self, x: u16) -> Result<&mut Self> {
        write!(self.stdout, "\r{}", Right(x))?;
        self.flush()
    }

    pub fn teardown(&mut self) -> Result<&mut Self> {
        util::redraw_window()?;
        self.reset()?;
        let start = self.start;
        self.write(start)
    }
}

impl Drop for Dropdown {
    fn drop(&mut self) {
        self.teardown().unwrap();
    }
}
