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

    fn current(&self) -> String {
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
        let mut n_lines = 0;
        let mut lines = self.values.iter();
        let max_lines = self.max_lines();

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

    fn padded(&self) -> String {
        format!("{} ", self.current())
    }

    pub fn prompt(&mut self) -> Option<String> {
        self.complete();

        // If there's only one option on the fist complete, then
        // assume it's correct
        if self.values.len() == 1 { return Some(self.padded()) }

        self.dropdown.reset();
        loop {
            match *self.prompt_next() {
                ReadEvent::Exit                => return None,
                ReadEvent::Submit              => return Some(self.padded()),
                ReadEvent::Tab                 => return Some(self.padded()),
                ReadEvent::Key(Key::Ctrl('n')) => self.values.forward(),
                ReadEvent::Key(Key::Down)      => self.values.forward(),
                ReadEvent::Key(Key::Ctrl('p')) => self.values.back(),
                ReadEvent::Key(Key::Up)        => self.values.back(),
                _                              => self.complete(),
            };
        }
    }
}
