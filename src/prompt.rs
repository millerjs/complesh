use ::completer::Completer;
use ::dropdown::Dropdown;
use ::errors::Result;
use ::filter::SpacedFilter;
use ::readkeys::{Readkeys, ReadEvent, Printable};
use ::ring_buffer::RingBuffer;
use std::path::PathBuf;
use termion::clear;
use termion::color::{self, Blue, Fg};
use termion::event::Key;

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
        self.values = self.completer.complete::<SpacedFilter>(&self.readkeys.value);
    }

    fn max_lines(&self) -> usize {
        (self.dropdown.height - 1) as usize
    }

    fn update_prompt(&mut self) {
        self.prompt = format!("{}{}: {}", Fg(Blue), self.completer.label(), Fg(color::Reset));
    }

    fn prompt_line(&mut self) -> String {
        format!("{}{}{}", clear::CurrentLine, self.prompt, self.readkeys.value)
    }

    fn render_prompt(&mut self) -> Result<()> {
        self.update_prompt();
        let prompt_line = self.prompt_line();
        self.dropdown.goto_origin()?.write(prompt_line)?.flush()?;
        let cursor = self.readkeys.cursor;
        self.dropdown.set_cursor((self.prompt.width() + cursor) as u16)?;
        Ok(())
    }

    fn render_dropdown(&mut self) -> Result<()> {
        let mut n_lines = 0;
        let lines = self.values.iter();
        let max_lines = self.max_lines();

        for line in lines.take(max_lines) {
            let prefix = if n_lines == 0 {"-> "} else {"   "};
            self.dropdown.writeln(format!("{}{}", prefix, line))?;
            n_lines += 1;
        }

        for _ in 0..(max_lines as i64 - n_lines) {
            self.dropdown.writeln("")?;
        }
        Ok(())
    }

    fn prompt_next<'a>(&'a mut self) -> Result<&'a ReadEvent> {
        self.render()?;
        Ok(self.readkeys.recv())
    }

    fn render(&mut self) -> Result<()> {
        self.render_dropdown()?;
        self.render_prompt()
    }

    fn padded(&self) -> String {
        format!("{} ", self.current())
    }

    fn readkeys_padded(&self) -> String {
        format!("{} ", self.readkeys.value)
    }

    fn toggle_mode(&mut self) {
        self.completer.toggle_mode();
        self.complete();
    }

    fn exit_on_tab(&self) -> bool {
        self.values.len() == 1 && !PathBuf::from(&self.current()).is_dir()
    }

    fn tab_to_dir(&mut self) {
        let current = self.current();
        if PathBuf::from(&current).is_dir() {
            self.readkeys.set_value(current)
        } else if let Some(first) = self.values.iter().next() {
            self.readkeys.set_value(first.without_escape_codes())
        }
        self.complete()
    }

    pub fn prompt(&mut self) -> Result<Option<String>> {
        self.complete();

        // If there's only one option on the first complete, then
        // assume it's correct
        if self.exit_on_tab() {
            return Ok(Some(self.padded()))
        }

        self.dropdown.reset()?;
        loop {
            match *self.prompt_next()? {
                ReadEvent::Exit                      => return Ok(None),
                ReadEvent::Submit                    => return Ok(Some(self.padded())),
                ReadEvent::Key(Key::Ctrl('j'))       => return Ok(Some(self.readkeys_padded())),
                ReadEvent::Key(Key::Ctrl('n'))       => self.values.forward(),
                ReadEvent::Key(Key::Down)            => self.values.forward(),
                ReadEvent::Key(Key::Ctrl('p'))       => self.values.back(),
                ReadEvent::Key(Key::Up)              => self.values.back(),
                ReadEvent::Key(Key::Null)            => self.toggle_mode(),
                ReadEvent::Tab if self.exit_on_tab() => return Ok(Some(self.padded())),
                ReadEvent::Tab                       => self.tab_to_dir(),
                _                                    => self.complete(),
            };
        }
    }
}
