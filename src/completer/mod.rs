use ::ring_buffer::RingBuffer;
use ::filter::Filter;

pub trait Completer {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String>;
}

mod recursive;
mod git;
mod mixed;
mod list;

pub use self::git::GitCompleter;
pub use self::recursive::RecursiveCompleter;
pub use self::list::ListCompleter;
pub use self::mixed::MixedCompleter;
