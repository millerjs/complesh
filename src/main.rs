extern crate complesh;
extern crate termion;

use termion::event::Key;
use termion::{color, clear, terminal_size};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, Stdout, stdout, stdin};
use termion::cursor::{Goto, Down};
use std::fmt::Display;
use std::cmp::min;

// static PREFIX_CURRENT: &'static str = "-> ";
static PREFIX_ELIPSIS: &'static str = "   ...";
// static PREFIX_ENTRY: &'static str = "   ";


pub struct Dropdown {
    pub stdout: RawTerminal<Stdout>,
    pub origin: Goto,
    pub height: u16,
}


impl Dropdown {
    fn new() -> Self {
        let mut out = stdout().into_raw_mode().unwrap();
        Self {
            origin: Goto(1, complesh::sync_cursor_pos(&mut out).unwrap().1),
            stdout: out,
            height: 4,
        }
    }

    fn goto_origin(&mut self) {
        write!(self.stdout, "{}", self.origin).unwrap();
    }

    pub fn reset(&mut self) {
        self.goto_origin();
        for _ in 0..self.height {
            self.write(format!("{}\n", clear::CurrentLine));
        }
        self.origin.1 = min(self.origin.1, terminal_size().unwrap().1 - self.height);
        self.goto_origin();
    }

    fn clearline(&mut self) {
        let y = complesh::sync_cursor_pos(&mut self.stdout).unwrap().1;
        write!(self.stdout, "{}{}", clear::CurrentLine, Goto(1, y)).unwrap();
    }

    fn write<D>(&mut self, value: D) where D: Display {
        write!(self.stdout, "{}", value).unwrap();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}


pub struct DropdownPrompt {
    dropdown: Dropdown,
}

impl DropdownPrompt {
    fn new() -> Self {
        Self { dropdown: Dropdown::new() }
    }

    fn prompt<D: Display>(&mut self, prompt: &String, value: &String, lines: &[D]) {
        let mut dropdown = &mut self.dropdown;

        dropdown.reset();
        for line in lines.iter().take((dropdown.height - 1) as usize) {
            dropdown.write(Down(1));
            dropdown.clearline();
            dropdown.write(format!("{}", line))
        }
        if lines.len() > dropdown.height as usize {
            dropdown.write(PREFIX_ELIPSIS)
        }
        dropdown.goto_origin();
        dropdown.write(format!("{}{}", prompt, value));
        dropdown.flush();
    }
}

fn main() {
    let mut popup = DropdownPrompt::new();
    let mut value = String::new();
    let prompt = format!("{}enter text: {}", color::Fg(color::Green), color::Fg(color::Reset));
    let stdin = stdin();

    popup.prompt(&prompt, &value, &["first", &*value, "third", "fourth"]);

    for c in stdin.keys() {
        match c.unwrap() {
            Key::Ctrl('c')  => break,
            Key::Ctrl('d')  => if value.len() == 0 { break },
            Key::Char('\n') => break,
            Key::Backspace  => { value.pop(); },
            Key::Ctrl('u')  => value = "".to_string(),
            Key::Char(c)    => value += &*c.to_string(),
            _               => (),
        }

        popup.prompt(&prompt, &value, &["first", &*value, "third", "fourth"]);
    }

    popup.dropdown.reset();
}
