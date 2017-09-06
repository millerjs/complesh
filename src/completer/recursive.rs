use ::completer::{Completer, CompleterBase};
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use ::util::{search_root, path_string, git_root};
use walkdir::WalkDir;
use std::path::Path;

pub struct RecursiveCompleter {
    max_depth: usize,
    max_git_depth: usize,
    follow_links: bool,
    base: CompleterBase,
}

impl Default for RecursiveCompleter {
    fn default() -> RecursiveCompleter {
        RecursiveCompleter {
            max_depth: 2,
            max_git_depth: 32,
            follow_links: false,
            base: CompleterBase::new(),
        }
    }
}

impl RecursiveCompleter {
    fn format_path<P: AsRef<Path>>(path: P) -> String {
        if path.as_ref().is_dir() {
            format!("{}/", path_string(path))
        } else {
            path_string(path)
        }
    }

    fn max_depth(&self, query: &str) -> usize {
        if !git_root(query).unwrap_or(String::new()).is_empty() {
            self.max_git_depth
        } else {
            self.max_depth
        }
    }
}

impl Completer for RecursiveCompleter {
    fn label(&self) -> String {
        "recursive".to_string()
    }

    fn complete<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        let root = path_string(search_root(query));
        let (links, depth) = (self.follow_links, self.max_depth(query));

        self.base.complete::<F, _>(query, &*root, || {
            WalkDir::new(&root)
                .follow_links(links)
                .max_depth(depth)
                .into_iter()
                .map(|p| RecursiveCompleter::format_path(p.unwrap().path()))
                .collect()
        })
    }
}
