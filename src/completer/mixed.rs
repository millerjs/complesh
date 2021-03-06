use ::completer::{Completer, GitCompleter, RecursiveCompleter};
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use ::util::{git_root, search_root, path_string, canonicalize};
use std::path::Path;

pub enum Mode {
    Git,
    Recursive,
    Auto,
}

pub struct MixedCompleter {
    git: GitCompleter,
    recursive: RecursiveCompleter,
    mode:  Mode,
    root: String,
}

impl Default for MixedCompleter {
    fn default() -> MixedCompleter {
        MixedCompleter {
            git: GitCompleter::default(),
            recursive: RecursiveCompleter::default(),
            mode: Mode::Auto,
            root: String::from("."),
        }
    }
}

impl MixedCompleter {
    pub fn mode(&mut self, mode: Mode) -> &mut Self {
        self.mode = mode;
        self
    }

    fn complete_git<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        if self.git_allowed() {
            self.git.complete::<F>(&*query)
        } else {
            RingBuffer::from_vec(vec![])
        }
    }

    fn complete_auto<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        if self.git_allowed() {
            self.complete_git::<F>(query)
        } else {
            self.recursive.complete::<F>(query)
        }
    }

    fn complete_recursive<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        self.recursive.complete::<F>(query)
    }

    fn git_allowed(&self) -> bool {
        !git_root(&*self.root).unwrap_or(String::new()).is_empty()
    }

    fn update_root<P: AsRef<Path>>(&mut self, query: P) {
        self.root = path_string(search_root(query));
        self.git.root = self.root.clone();
    }
}

impl Completer for MixedCompleter {
    fn label(&self) -> String {
        match self.mode {
            Mode::Auto      => if self.git_allowed() { "auto [git]" } else { "auto [rec]" },
            Mode::Git       => "git",
            Mode::Recursive => "recursive",
        }.to_string()
    }

    fn toggle_mode(&mut self) {
        self.mode = match self.mode {
            Mode::Recursive => Mode::Git,
            Mode::Git       => Mode::Auto,
            Mode::Auto      => Mode::Recursive,
        };
    }

    fn complete<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        self.update_root(query);
        let query = &*path_string(canonicalize(query));

        match self.mode {
            Mode::Auto      => self.complete_auto::<F>(query),
            Mode::Git       => self.complete_git::<F>(query),
            Mode::Recursive => self.complete_recursive::<F>(query),
        }
    }
 }
