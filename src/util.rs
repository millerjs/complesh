use ::errors::Result;
use nix::sys::signal;
use nix::unistd;
use std::env::home_dir;
use std::env;
use std::fmt::Display;
use std::io::{self, Write, Stdout, Read, stdin};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::time::{SystemTime, Duration};
use termion::color::{self, Green, Fg};
use termion::raw::CONTROL_SEQUENCE_TIMEOUT;
use termion::style::{self, Underline, Bold};
use termion::terminal_size;

pub fn log<D>(value: D) where D: Display {
    use std::io::prelude::*;
    use std::fs::OpenOptions;
    let path = "/var/log/complesh/complesh.log";
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
pub fn redraw_window() -> Result<()> {
    Ok(signal::kill(unistd::getppid(), signal::Signal::SIGWINCH)?)
}


pub fn window_height() -> Result<u16> {
    Ok(terminal_size()?.1)
}

pub fn expand_user<P: AsRef<Path>>(path: P) -> PathBuf {
    if let Ok(relative_path) = path.as_ref().strip_prefix("~/") {
        if let Some(home) = home_dir() {
            return home.join(relative_path)
        }
    }
    path.as_ref().to_owned()
}

pub fn emphasize<D: Display>(value: D) -> String {
    format!("{}{}{}{}{}{}", Fg(Green), Underline, Bold, value, Fg(color::Reset), style::Reset)
}

pub fn within_dir<F: FnOnce() -> T, P: AsRef<Path>, T>(path: P, f: F) -> Result<T> {
    let cwd = env::current_dir()?;
    env::set_current_dir(path)?;
    let result = f();
    env::set_current_dir(cwd)?;
    Ok(result)
}

pub fn git_root<P: AsRef<Path>>(path: P) -> Result<String> {
    within_dir(path.as_ref(), || {
        Command::new("git").arg("rev-parse").arg("--show-toplevel").output()
            .map(|output| String::from_utf8(output.stdout))
            .and_then(|path| Ok(path.unwrap().trim().to_string()))
            .unwrap_or(String::new())
    })
}

pub fn absolute_path<P: AsRef<Path>>(path: P) -> PathBuf {
    if let Ok(cwd) = env::current_dir() {
        cwd.join(path.as_ref())
    } else {
        path.as_ref().to_owned()
    }
}

pub fn strip_root<R: AsRef<Path>, P: AsRef<Path>>(root: R, path: P) -> PathBuf {
    match path.as_ref().strip_prefix(root.as_ref()) {
        Ok(relative_path) => relative_path.to_owned(),
        _ => path.as_ref().to_owned(),
    }
}

pub fn canonicalize<P: AsRef<Path>>(path: P) -> PathBuf {
    let path = path.as_ref().to_owned();
    if let Ok(canonical) = path.canonicalize() {
        canonical.to_owned()
    } else {
        path
    }
}

pub fn path_string<P: AsRef<Path>>(path: P) -> String {
    path.as_ref().to_string_lossy().to_string()
}

pub fn search_root<P: AsRef<Path>>(path: P) -> PathBuf {
    let expanded = canonicalize(expand_user(path));
    if expanded.is_dir() {
        return expanded
    }
    if let Some(parent) = expanded.parent() {
        if parent.is_dir() {
            return parent.to_owned()
        }
    }
    PathBuf::from(".")
}

#[test]
fn test_git_root() {
    assert!(git_root(".").is_ok());
}
