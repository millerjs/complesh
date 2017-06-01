use termion::cursor::{Goto, Right, Down};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, terminal_size};
use std::io::{Write, Stdout, stdout};
use std::fmt::Display;
use std::cmp::{max, min};

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

    pub fn goto_origin(&mut self) -> &mut Self {
        write!(self.stdout, "{}", self.origin).unwrap();
        self
    }

    pub fn resize(&mut self) -> &mut Self {
        self.height = max(MIN_HEIGHT, min(util::window_height() - self.start.1, self.max_height));
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.resize().goto_origin();
        for _ in 0..(self.height+1) {
            self.write(format!("{}\n", clear::CurrentLine));
        }
        self.origin.1 = min(self.origin.1, terminal_size().unwrap().1 - self.height);
        self.goto_origin();
        self
    }

    pub fn clearline(&mut self) -> &mut Self {
        write!(self.stdout, "{}\r", clear::CurrentLine).unwrap();
        self
    }

    pub fn write<D>(&mut self, value: D) -> &mut Self where D: Display {
        write!(self.stdout, "{}", value).unwrap();
        self
    }

    pub fn writeln<D>(&mut self, value: D) -> &mut Self where D: Display {
        self.write(Down(1)).clearline().write(value)
    }

    pub fn flush(&mut self) -> &mut Self {
        self.stdout.flush().unwrap();
        self
    }

    pub fn set_cursor(&mut self, x: u16) {
        write!(self.stdout, "\r{}", Right(x)).unwrap();
        self.flush();
    }

    pub fn teardown(&mut self) {
        util::redraw_window();
        self.reset();
        let start = self.start;
        self.write(start);
    }
}

impl Drop for Dropdown {
    fn drop(&mut self) {
        self.teardown()
    }
}
