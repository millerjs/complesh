use std::fmt::Display;
use termion::color::{self, Green, Fg};
use termion::style::{self, Underline, Bold};
use ::ring_buffer::RingBuffer;

pub trait Completer {
    fn complete(&mut self, query: &str, limit: usize) -> RingBuffer<String>;
}

fn emphasize<D: Display>(value: D) -> String {
    format!("{}{}{}{}{}{}", Fg(Green), Underline, Bold, value, Fg(color::Reset), style::Reset)
}

mod glob;
mod recursive;

pub use self::recursive::RecursiveCompleter;
pub use self::glob::GlobCompleter;
