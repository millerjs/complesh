use std::fs::File;
use std::fmt::Display;
use termion::cursor::{Goto, Down, Right};

use ::dropdown::Dropdown;
use ::completer::Completer;
use ::readkeys::{Readkeys, ReadEvent, Printable};

static PREFIX_ELIPSIS: &str = "   ...";

pub struct DropdownPrompt<C: Completer> {
    dropdown: Dropdown,
    completer: C,
    prompt: String,
    readkeys: Readkeys,
}

impl<C> DropdownPrompt<C> where C: Completer {
    pub fn new(prompt: String, readkeys: Readkeys, dropdown: Dropdown, completer: C) -> Self {
        Self { prompt, readkeys, dropdown, completer }
    }

    pub fn render<D: Display>(&mut self, lines: &[D]) {
        let max_lines = (self.dropdown.max_height - 1) as usize;
        let mut lines = lines.iter();

        if let Some(line) = lines.next() {
            self.dropdown.write(Down(1)).clearline().write(format!("-> {}", line));
        }

        for line in lines.take(max_lines) {
            self.dropdown.write(Down(1)).clearline().write(format!("   {}", line));
        }
    }

    pub fn prompt(&mut self, beginning: &String) -> String {
        let max_lines = (self.dropdown.max_height - 1) as usize;
        self.dropdown.goto_origin().write(format!("{}{}", self.prompt, beginning)).flush();
        let value = self.readkeys.value.clone();
        self.render(&*C::complete(value, max_lines));

        loop {
            match *self.readkeys.recv() {
                ReadEvent::Exit | ReadEvent::Submit | ReadEvent::Tab => break,
                _ => ()
            };
            let value = self.readkeys.value.clone();
            self.render(&*C::complete(value, max_lines));
            let cursor = self.readkeys.cursor;
            self.dropdown.set_cursor((self.prompt.width() + cursor) as u16);
        }

        self.readkeys.value.clone()
    }
}
