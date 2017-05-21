use nlp_tokenize::{WhitePunctTokenizer, Tokenizer};
use regex;
use std::cmp::{min, max};
use std::io::{self, stdin};
use std::sync::mpsc::Receiver;
use std::sync::mpsc::channel;
use std::thread;
use termion::event::Key;
use termion::input::TermRead;

pub struct ReadkeysState {
    value: String,
    cursor: usize,
}

pub struct RingBuffer<T> {
    values: Vec<T>,
    cursor: usize,
}

pub struct Readkeys {
    pub value: String,
    pub cursor: usize,
    keys: Receiver<Result<Key, io::Error>>,
    tokenizer: WhitePunctTokenizer,
    state_history: Vec<ReadkeysState>,
    kill_ring: RingBuffer<String>,
    last_event: ReadEvent,
}

pub enum ReadEvent {
    Exit,
    Submit,
    CursorMove,
    Tab,
    Other,
    Yank,
    Key(Key),
}

pub trait Printable {
    fn width(&self) -> usize;
    fn without_escape_codes(&self) -> Self;
}


impl Printable for String {
    fn width(&self) -> usize {
        self.without_escape_codes().chars().count()
    }

    fn without_escape_codes(&self) -> Self {
        regex::Regex::new(r"\x1b\[[;\d]*[A-Za-z]").unwrap().replace_all(self, "").to_string()
    }
}

pub fn async_keys() -> Receiver<Result<Key, io::Error>> {
    let (tx, rx) = channel();
    thread::spawn(move|| {
        for c in stdin().keys() {
            if let Err(_) =  tx.send(c) { break }
        }
    });
    rx
}

pub enum Goto {
    BeginningOfLine,
    EndOfLine,
    BackwardsCharacter,
    BackwardsWord,
    ForwardsCharacter,
    ForwardsWord,
}


impl<T: Clone> RingBuffer<T> {
    pub fn new() -> Self {
        Self { values: Vec::new(), cursor: 0 }
    }

    pub fn insert(&mut self, value: T) {
        self.rotate();
        self.values.insert(self.cursor, value);
    }

    pub fn rotate(&mut self) {
        self.cursor += 1;
        if self.cursor >= self.values.len() {
            self.cursor = 0;
        }
    }

    pub fn clone_current(&self) -> Option<T> {
        match self.values.len() {
            0 => None,
            _ => Some(self.values[self.cursor].clone()),
        }
    }
}

impl Readkeys {
    pub fn new<S: Into<String>>(value: S) -> Self {
        let value = value.into();
        Self {
            cursor: value.len(),
            keys: async_keys(),
            tokenizer: WhitePunctTokenizer::new(),
            state_history: Vec::new(),
            kill_ring: RingBuffer::new(),
            last_event: ReadEvent::Other,
            value,
        }
    }

    pub fn recv<'a>(&'a mut self) -> &'a ReadEvent {
        let key = self.keys.recv().unwrap().unwrap();
        let mut event = ReadEvent::Key(key);
        match key {
            Key::Ctrl('c')     => event = ReadEvent::Exit,
            Key::Char('\n')    => event = ReadEvent::Submit,
            Key::Char('\t')    => event = ReadEvent::Tab,
            Key::Ctrl('d')     => if self.value.len() == 0 { event = ReadEvent::Exit },
            Key::Backspace     => self.backspace(),
            Key::Ctrl('h')     => self.backspace(),
            Key::Ctrl('e')     => self.move_cursor(Goto::EndOfLine),
            Key::Ctrl('a')     => self.move_cursor(Goto::BeginningOfLine),
            Key::Ctrl('b')     => self.move_cursor(Goto::BackwardsCharacter),
            Key::Left          => self.move_cursor(Goto::BackwardsCharacter),
            Key::Alt('b')      => self.move_cursor(Goto::BackwardsWord),
            Key::Alt('f')      => self.move_cursor(Goto::ForwardsWord),
            Key::Ctrl('f')     => self.move_cursor(Goto::ForwardsCharacter),
            Key::Right         => self.move_cursor(Goto::ForwardsCharacter),
            Key::Alt('\u{7f}') => self.backspace_word(),
            Key::Ctrl('7')     => self.pop_state(),
            Key::Ctrl('u')     => self.kill_before_cursor(),
            Key::Ctrl('k')     => self.kill_after_cursor(),
            Key::Char(c)       => self.write(&*c.to_string()),
            Key::Ctrl('y')     => { self.yank();      event = ReadEvent::Yank },
            Key::Alt('y')      => { self.yank_next(); event = ReadEvent::Yank },
            _                  => (),
        };
        self.last_event = event;
        &self.last_event
    }

    fn push_state(&mut self) {
        self.state_history.push(ReadkeysState { value: self.value.clone(), cursor: self.cursor });
    }

    pub fn yank(&mut self) {
        if let Some(yanked) = self.kill_ring.clone_current() {
            self.push_state();
            self.write(&*yanked);
            self.last_event = ReadEvent::Yank;
        }
    }

    pub fn yank_next(&mut self) {
        if let ReadEvent::Yank = self.last_event {
            self.pop_state();
            self.kill_ring.rotate();
            self.yank();
        }
    }

    pub fn pop_state(&mut self) {
        if let Some(state) = self.state_history.pop() {
            self.value = state.value;
            self.cursor = state.cursor;
        } else {
            self.value = String::new();
            self.cursor = 0;
        }
    }

    pub fn backspace(&mut self) {
        self.push_state();
        if self.cursor > 0 {
            self.value.remove(self.cursor-1);
            self.cursor -= 1;
        }
    }

    pub fn backspace_word(&mut self) {
        self.push_state();
        let start = self.previous_word_start();
        self.kill_ring.insert(self.value[start..self.cursor].to_string());
        self.value = format!("{}{}", &self.value[..start], &self.value[self.cursor..]);
        self.cursor = start;
    }

    pub fn kill_before_cursor(&mut self) {
        self.push_state();
        self.kill_ring.insert(self.value[..self.cursor].to_string());
        self.value = self.value[self.cursor..].to_string();
        self.cursor = 0;
    }

    pub fn kill_after_cursor(&mut self) {
        self.push_state();
        self.kill_ring.insert(self.value[self.cursor..].to_string());
        self.value = self.value[..self.cursor].to_string();
    }

    pub fn write(&mut self, value: &str) {
        self.value.insert_str(self.cursor, &*value);
        self.cursor += value.len();
    }

    fn increment_cursor(&mut self, distance: i32) {
        self.cursor = self.bound_cursor(self.cursor as i32 + distance);
    }

    fn bound_cursor(&self, cursor: i32) -> usize{
        min(self.value.len(), max(0, cursor) as usize) as usize
    }

    fn previous_word_start(&self) -> usize {
        let tokens = self.tokenizer.tokenize(&self.value[..self.cursor]);
        if tokens.len() > 0 { tokens[tokens.len() - 1].0 } else { self.cursor }
    }

    fn next_word_end(&self) -> usize {
        let tokens = self.tokenizer.tokenize(&self.value[self.cursor..]);
        self.cursor + if tokens.len() > 0 { tokens[0].1 } else { 0 }
    }

    pub fn move_cursor(&mut self, to: Goto) {
        match to {
            Goto::BeginningOfLine    => self.cursor = 0,
            Goto::EndOfLine          => self.cursor = self.value.len(),
            Goto::BackwardsCharacter => self.increment_cursor(-1),
            Goto::BackwardsWord      => self.cursor = self.previous_word_start(),
            Goto::ForwardsCharacter  => self.increment_cursor(1),
            Goto::ForwardsWord       => self.cursor = self.next_word_end(),
        }
    }

    pub fn set_value(&mut self, value: String) {
        self.value = value;
        self.move_cursor(Goto::EndOfLine);
    }
}


#[cfg(test)]
mod test {
    use termion::color;
    use readkeys::Printable;

    #[test]
    fn test_printable_string_strip_escape_characters() {
        let actual = format!("{}123", color::Fg(color::Blue)).without_escape_codes();
        let expected = "123".to_string();
        assert_eq!(actual, expected)
    }

    #[test]
    fn test_printable_string_width() {
        let actual = format!("{}123", color::Fg(color::Blue)).width();
        assert_eq!(actual, 3)
    }

}
