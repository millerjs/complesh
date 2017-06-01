use ::completer::{Completer, GitCompleter, GlobCompleter};
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use ::util::git_root;

pub struct MixedCompleter {
    git: GitCompleter,
    glob: GlobCompleter,
}

impl MixedCompleter {
    pub fn new() -> Self {
        Self { git: GitCompleter::new(), glob: GlobCompleter{} }
    }
}

impl Completer for MixedCompleter {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        if git_root(".").is_some() {
            self.git.complete::<F>(query, limit)
        } else {
            self.glob.complete::<F>(query, limit)
        }
    }
}
