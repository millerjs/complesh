use ::completer::{Completer, GitCompleter, RecursiveCompleter};
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use ::util::git_root;

pub enum Mode {
    Git,
    Recursive,
    Auto,
}

pub struct MixedCompleter {
    git: GitCompleter,
    recursive: RecursiveCompleter,
    mode:  Mode,
}

impl MixedCompleter {
    pub fn new() -> Self {
        Self {
            git: GitCompleter::new(),
            recursive: RecursiveCompleter::default(),
            mode: Mode::Auto,
        }
    }

    pub fn mode(&mut self, mode: Mode) -> &mut Self {
        self.mode = mode;
        self
    }

    fn complete_git<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        if self.git_allowed() {
            self.git.complete::<F>(query, limit)
        } else {
            RingBuffer::from_vec(vec![])
        }
    }

    fn complete_auto<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        if self.git_allowed() {
            self.git.complete::<F>(query, limit)
        } else {
            self.recursive.complete::<F>(query, limit)
        }
    }

    fn complete_recursive<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        self.recursive.complete::<F>(query, limit)
    }

    fn git_allowed(&self) -> bool {
        !git_root(".").unwrap_or(String::new()).is_empty()
    }
}

impl Completer for MixedCompleter {
    fn label(&self) -> String {
        match self.mode {
            Mode::Auto      => if self.git_allowed() { "auto (git)" } else { "auto (recursive)" },
            Mode::Git       => "git",
            Mode::Recursive => "recursive",
        }.to_string()
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Recursive => Mode::Git,
            _               => Mode::Recursive,
        };
    }

    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        match self.mode {
            Mode::Auto      => self.complete_auto::<F>(query, limit),
            Mode::Git       => self.complete_git::<F>(query, limit),
            Mode::Recursive => self.complete_recursive::<F>(query, limit),
        }
    }
 }
