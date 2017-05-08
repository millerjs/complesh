use termion::cursor::{Goto, Down, Right};
use termion::raw::{IntoRawMode, RawTerminal};
use termion::{clear, terminal_size};
use std::io::{self, Write, Stdout, Read, stdout, stdin};
use std::fmt::Display;
use std::cmp::{min};
use std::time::{SystemTime, Duration};
use termion::raw::CONTROL_SEQUENCE_TIMEOUT;

static PREFIX_ELIPSIS: &str = "   ...";


pub struct Dropdown {
    stdout: RawTerminal<Stdout>,
    start: Goto,
    origin: Goto,
    max_height: u16,
    height: u16,
}

impl Dropdown {
    pub fn new(height: u16) -> Self {
        let mut out = stdout().into_raw_mode().unwrap();
        let (x, y) = sync_cursor_pos(&mut out).unwrap();
        let origin = if x == 1 { Goto(1, y) } else { Goto(1, y+1) };
        Self {
            start: Goto(x, y),
            stdout: out,
            height: height,
            max_height: height,
            origin
        }
    }

    pub fn goto_origin(&mut self) -> &mut Self {
        write!(self.stdout, "{}", self.origin).unwrap();
        self
    }

    pub fn resize(&mut self) -> &mut Self {
        self.height = min(terminal_size().unwrap().1, self.max_height);
        self
    }

    pub fn reset(&mut self) -> &mut Self {
        self.resize().goto_origin();
        for _ in 0..self.height {
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

    pub fn flush(&mut self) -> &mut Self {
        self.stdout.flush().unwrap();
        self
    }

    pub fn set_cursor(&mut self, x: u16) {
        write!(self.stdout, "\r{}", Right(x)).unwrap();
        self.flush();
    }

    pub fn teardown(&mut self) {
        self.reset();
        let start = self.start;
        self.write(start);
    }
}


pub struct DropdownPrompt {
    pub dropdown: Dropdown,
}


impl DropdownPrompt {
    pub fn new(dropdown: Dropdown) -> Self {
        Self { dropdown: dropdown }
    }

    pub fn prompt<D: Display>(&mut self, prompt: &String, value: &String, options: &[D]) {
        let max_lines = (self.dropdown.height - 1) as usize;
        let mut dropdown = &mut self.dropdown.reset();

        for line in options.iter().take(max_lines) {
            dropdown.write(Down(1)).clearline().write(format!("   {}", line));
        }

        if options.len() > dropdown.height as usize {
            dropdown.write(PREFIX_ELIPSIS);
        }

        dropdown.goto_origin().write(format!("{}{}", prompt, value)).flush();
    }
}


pub fn sync_cursor_pos(stdout: &mut Stdout) -> io::Result<(u16, u16)> {
    let mut stdin = stdin();

    // Where is the cursor?
    // Use `ESC [ 6 n`.
    write!(stdout, "\x1B[6n")?;
    stdout.flush()?;

    let mut buf: [u8; 1] = [0];
    let mut read_chars = Vec::new();

    let timeout = Duration::from_millis(CONTROL_SEQUENCE_TIMEOUT);
    let now = SystemTime::now();

    // Either consume all data up to R or wait for a timeout.
    while buf[0] != b'R' && now.elapsed().unwrap() < timeout {
        if stdin.read(&mut buf)? > 0 {
            read_chars.push(buf[0]);
        }
    }

    // The answer will look like `ESC [ Cy ; Cx R`.

    read_chars.pop(); // remove trailing R.
    let read_str = String::from_utf8(read_chars).unwrap();
    let beg = read_str.rfind('[').unwrap();
    let coords: String = read_str.chars().skip(beg + 1).collect();
    let mut nums = coords.split(';');

    let cy = nums.next().unwrap().parse().unwrap();
    let cx = nums.next().unwrap().parse().unwrap();

    Ok((cx, cy))
}
