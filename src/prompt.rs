use std::fmt::Display;
use termion::cursor::Down;
use termion::clear;

use ::dropdown::Dropdown;
use ::completer::Completer;
use ::readkeys::{Readkeys, ReadEvent, Printable};

pub struct DropdownPrompt<C: Completer> {
    dropdown: Dropdown,
    prompt: String,
    readkeys: Readkeys,
    completer: Box<C>,
}

impl<C> DropdownPrompt<C> where C: Completer {
    pub fn new(prompt: String, readkeys: Readkeys, dropdown: Dropdown, completer: Box<C>) -> Self {
        Self { prompt, readkeys, dropdown, completer }
    }

    fn current(&mut self) -> String {
        self.complete().iter().next().unwrap_or(&self.readkeys.value).clone().without_escape_codes()
    }

    fn complete(&mut self) -> Vec<String> {
        let max_lines = self.max_lines();
        self.completer.complete(&self.readkeys.value, max_lines)
    }

    fn max_lines(&self) -> usize {
        (self.dropdown.max_height - 1) as usize
    }

    fn render_prompt(&mut self) {
        let prompt_line = format!("{}{}{}", clear::CurrentLine, self.prompt, self.readkeys.value);
        self.dropdown.goto_origin().write(prompt_line).flush();
        let cursor = self.readkeys.cursor;
        self.dropdown.set_cursor((self.prompt.width() + cursor) as u16);
    }

    fn render_dropdown<D: Display>(&mut self, lines: &[D]) {
        let max_lines = self.max_lines();
        let mut lines_iter = lines.iter();

        if let Some(line) = lines_iter.next() {
            self.dropdown.write(Down(1)).clearline().write(format!("-> {}", line));
        }

        for line in lines_iter.take(max_lines) {
            self.dropdown.write(Down(1)).clearline().write(format!("   {}", line));
        }

        for _ in 0..(max_lines - lines.len()) {
            self.dropdown.write(Down(1)).clearline();
        }
    }

    fn prompt_next<'a>(&'a mut self) -> &'a ReadEvent {
        let values = self.complete();
        self.render_dropdown(&*values);
        self.render_prompt();
        self.readkeys.recv()
    }

    pub fn prompt(&mut self) -> Option<String> {
        self.dropdown.goto_origin();

        loop {
            match *self.prompt_next() {
                ReadEvent::Exit => return None,
                ReadEvent::Submit => return Some(self.current()),
                ReadEvent::Tab => {
                    let value = self.current();
                    self.readkeys.set_value(value);
                    self.render_prompt();
                }, _ => ()
            };
        }
    }
}
