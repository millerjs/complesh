use ::ring_buffer::RingBuffer;
use ::filter::Filter;

pub trait Completer {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String>;
}

mod glob;
mod recursive;

pub use self::recursive::RecursiveCompleter;
pub use self::glob::GlobCompleter;
