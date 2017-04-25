#[macro_use] extern crate quick_error;
extern crate termion;

mod errors;

use std::time::{SystemTime, Duration};
use termion::raw::CONTROL_SEQUENCE_TIMEOUT;
use std::io::{self, Write, Read, Stdout, stdin};

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
