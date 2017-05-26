use nix::sys::signal;
use nix::unistd;
use std::io::{self, Write, Stdout, Read, stdin};
use std::time::{SystemTime, Duration};
use termion::raw::CONTROL_SEQUENCE_TIMEOUT;
use termion::terminal_size;
use std::fmt::Display;
use termion::color::{self, Green, Fg};
use termion::style::{self, Underline, Bold};
use std::env::home_dir;

pub fn log<D>(value: D) where D: Display {
    use std::io::prelude::*;
    use std::fs::OpenOptions;
    let path = "complesh.log";
    let mut file = OpenOptions::new().write(true).create(true).append(true).open(path).unwrap();
    file.write_all(format!("{}", value).as_bytes()).unwrap();
    file.flush().unwrap();
}

/// Vendor this function with a small modification to avoid panic
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


/// Sends SIGWINCH to parent process to get it to redraw as necessary
pub fn redraw_window() {
    signal::kill(unistd::getppid(), signal::Signal::SIGWINCH).unwrap();
}


pub fn window_height() -> u16 {
    terminal_size().unwrap().1
}

pub fn expand_user(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = home_dir() {
            home.display().to_string() + &path[2..]
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    }
}

pub fn emphasize<D: Display>(value: D) -> String {
    format!("{}{}{}{}{}{}", Fg(Green), Underline, Bold, value, Fg(color::Reset), style::Reset)
}
