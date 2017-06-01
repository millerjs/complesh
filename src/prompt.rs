use termion::clear;
use termion::event::Key;
use ::dropdown::Dropdown;
use ::completer::Completer;
use ::readkeys::{Readkeys, ReadEvent, Printable};
use ::ring_buffer::RingBuffer;
use ::filter::SpacedFilter;

pub struct DropdownPrompt<C: Completer> {
    dropdown: Dropdown,
    prompt: String,
    readkeys: Readkeys,
    completer: Box<C>,
    values: RingBuffer<String>,
}

impl<C> DropdownPrompt<C> where C: Completer {
    pub fn new(prompt: String, readkeys: Readkeys, dropdown: Dropdown, completer: Box<C>) -> Self {
        Self { values: RingBuffer::new(), prompt, readkeys, dropdown, completer }
    }

    fn current(&mut self) -> String {
        self.values.current().unwrap_or(&self.readkeys.value).clone().without_escape_codes()
    }

    fn complete(&mut self) {
        let max_lines = self.max_lines();
        self.values = self.completer.complete::<SpacedFilter>(&self.readkeys.value, max_lines);
    }

    fn max_lines(&self) -> usize {
        (self.dropdown.height - 1) as usize
    }

    fn render_prompt(&mut self) {
        let prompt_line = format!("{}{}{}", clear::CurrentLine, self.prompt, self.readkeys.value);
        self.dropdown.goto_origin().write(prompt_line).flush();
        let cursor = self.readkeys.cursor;
        self.dropdown.set_cursor((self.prompt.width() + cursor) as u16);
    }

    fn render_dropdown(&mut self) {
        let mut lines = self.values.iter();
        let max_lines = self.max_lines();
        let mut n_lines = 0;

        if let Some(line) = lines.next() {
            self.dropdown.writeln(format!("-> {}", line));
            n_lines += 1;
        }

        for line in lines.take(max_lines) {
            self.dropdown.writeln(format!("   {}", line));
            n_lines += 1;
        }

        if n_lines < max_lines {
            for _ in 0..(max_lines - n_lines) {
                self.dropdown.writeln("");
            }
        }
    }

    fn prompt_next<'a>(&'a mut self) -> &'a ReadEvent {
        self.render_dropdown();
        self.render_prompt();
        self.readkeys.recv()
    }

    pub fn prompt(&mut self) -> Option<String> {
        self.dropdown.reset();
        self.complete();
        let mut tabbed = false;

        loop {
            match *self.prompt_next() {
                ReadEvent::Exit => return None,
                ReadEvent::Submit => return Some(self.current()),
                ReadEvent::Tab => {
                    if tabbed { return Some(self.current()) }
                    tabbed = true;
                    let value = self.current();
                    self.readkeys.set_value(value);
                    self.render_prompt();
                },
                ReadEvent::Key(Key::Down) | ReadEvent::Key(Key::Ctrl('n')) => {
                    tabbed = false;
                    self.values.forward();
                },
                ReadEvent::Key(Key::Up) | ReadEvent::Key(Key::Ctrl('p')) => {
                    tabbed = false;
                    self.values.back();
                },
                _ => {
                    tabbed = false;
                    self.complete();
                }
            };
        }
    }
}
