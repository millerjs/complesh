extern crate complesh;
extern crate termion;

use termion::event::Key;
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, Stdout, Stdin, stdout, stdin};
use termion::cursor::{Goto, Down};
use std::fmt::Debug;

macro_rules! write_down {
    ($dst:expr, $fmt:expr) => {{
        let y = Goto(1, complesh::sync_cursor_pos(&mut $dst).unwrap().1);
        write!($dst, "{}", termion::clear::CurrentLine).unwrap();
        write!($dst, concat!("{}", $fmt), y).unwrap();
        write!($dst, "{}", Down(1)).unwrap();
    }};
    ($dst:expr, $fmt:expr, $($arg:tt)*) => {{
        let y = Goto(1, complesh::sync_cursor_pos(&mut $dst).unwrap().1);
        write!($dst, "{}", termion::clear::CurrentLine).unwrap();
        write!($dst, concat!("{}", $fmt), y, $($arg)*).unwrap();
        write!($dst, "{}", Down(1)).unwrap();
    }};
}

pub struct Terminal {
    pub stdout: RawTerminal<Stdout>,
    pub start_x: u16,
    pub start_y: u16,
    pub limit: u16,
}


impl Terminal {
    fn new() -> Terminal {
        let mut out = stdout().into_raw_mode().unwrap();
        let (start_x, mut start_y) = complesh::sync_cursor_pos(&mut out).unwrap();
        Terminal {
            stdout: out,
            start_x: start_x,
            start_y: start_y,
            limit: 4,
        }
    }

    fn write_all<D: Debug>(&mut self, current: &String, values: &Vec<D>, limit: usize) {
        write_down!(self.stdout, "{}{}", Goto(1, self.start_y), current);
        write_down!(self.stdout, "-> {:?}", values.iter().next());
        for value in values.iter().skip(1).take(limit-1) {
            write_down!(self.stdout, "   {:?}", value);
        }
        write_down!(self.stdout, "{}", Goto(current.len() as u16, self.start_y-1));
        self.stdout.flush().unwrap();
    }

    fn initialize(&mut self) {
        let (width, height) = termion::terminal_size().unwrap();
        while self.start_y > height - self.limit + 1 {
            write!(self.stdout, "\n");
            self.start_y -= 1;
        }
    }

    fn teardown(&mut self) {
        write!(self.stdout, "{}", Goto(self.start_x, self.start_y));
        for _ in 0..self.limit {
            write_down!(self.stdout, "{}", termion::clear::CurrentLine);
        }
    }
}


fn main() {
    let mut terminal = Terminal::new();
    let mut value = String::new();
    let stdin = stdin();

    terminal.initialize();
    terminal.write_all(&value, &vec![""], 3);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c')  => break,
            Key::Char('\n') => (),
            Key::Char(c)    => value += &*c.to_string(),
            _               => (),
        }

        terminal.write_all(&value, &vec![
            "first",
            "second",
            "third",
            "fourth",
        ], 4);
    }

    terminal.teardown();
}
