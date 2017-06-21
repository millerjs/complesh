use ::completer::{Completer, GitCompleter, RecursiveCompleter};
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use ::util::git_root;

pub struct MixedCompleter {
    git: GitCompleter,
    recursive: RecursiveCompleter,
}

impl MixedCompleter {
    pub fn new() -> Self {
        Self { git: GitCompleter::new(), recursive: RecursiveCompleter::default() }
    }
}

impl Completer for MixedCompleter {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        if git_root(".").unwrap_or(String::new()).is_empty() {
            self.recursive.complete::<F>(query, limit)
        } else {
            self.git.complete::<F>(query, limit)
        }
    }
}
