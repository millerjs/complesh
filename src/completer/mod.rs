use ::ring_buffer::RingBuffer;
use ::filter::Filter;

pub trait Completer {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String>;
}

mod glob;
mod git;
mod mixed;

pub use self::git::GitCompleter;
pub use self::glob::GlobCompleter;
pub use self::mixed::MixedCompleter;
