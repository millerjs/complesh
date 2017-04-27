extern crate complesh;
extern crate termion;

use termion::event::Key;
use termion::{color, clear};
use termion::input::TermRead;
use termion::raw::{IntoRawMode, RawTerminal};
use std::io::{Write, Stdout, stdout, stdin};
use termion::cursor::{Goto, Down};
use std::fmt::Display;


// static PREFIX_CURRENT: &'static str = "-> ";
static PREFIX_ELIPSIS: &'static str = "   ...";
// static PREFIX_ENTRY: &'static str = "   ";


pub struct PopupPrompt {
    pub stdout: RawTerminal<Stdout>,
    pub start_x: u16,
    pub start_y: u16,
    pub height: u16,
}


impl PopupPrompt {
    fn new() -> Self {
        let mut out = stdout().into_raw_mode().unwrap();
        let (start_x, start_y) = complesh::sync_cursor_pos(&mut out).unwrap();
        Self {
            stdout: out,
            start_x: start_x,
            start_y: start_y,
            height: 4,
        }
    }

    fn goto_origin(&mut self) {
        write!(self.stdout, "{}", Goto(1, self.start_y)).unwrap();
    }

    pub fn clear(&mut self) {
        self.goto_origin();
        for _ in 0..self.height+2 {
            self.write(clear::CurrentLine);
        }
        self.goto_origin();
    }

    fn clearline(&mut self) {
        let y = complesh::sync_cursor_pos(&mut self.stdout).unwrap().1;
        write!(self.stdout, "{}{}", Goto(1, y), clear::CurrentLine).unwrap();
    }

    fn write<D>(&mut self, value: D) where D: Display {
        write!(self.stdout, "{}", value).unwrap();
    }

    fn max_lines(&self) -> usize {
        (self.height-1) as usize
    }

    fn prompt<D: Display>(&mut self, prompt: &String, value: &String, lines: &[D]) {
        self.clear();
        for line in lines.iter().take(self.max_lines()) {
            self.write(Down(1));
            self.clearline();
            self.write(format!("{}", line))
        }
        if lines.len() > self.max_lines() {
            self.write(PREFIX_ELIPSIS)
        }
        self.goto_origin();
        self.write(format!("{}{}", prompt, value));
        self.flush();
    }

    fn flush(&mut self) {
        self.stdout.flush().unwrap();
    }
}


fn main() {
    let mut popup = PopupPrompt::new();
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

    popup.clear();
}
