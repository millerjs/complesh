use ::completer::{Completer, CompleterBase};
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use ::util::{search_root, path_string};
use walkdir::WalkDir;
use std::path::Path;

pub struct RecursiveCompleter {
    max_depth: usize,
    follow_links: bool,
    base: CompleterBase,
}

impl Default for RecursiveCompleter {
    fn default() -> RecursiveCompleter {
        RecursiveCompleter {
            max_depth: 1,
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
}

impl Completer for RecursiveCompleter {
    fn label(&self) -> String {
        "recursive".to_string()
    }

    fn complete<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        let root = path_string(search_root(query));
        let (links, depth) = (self.follow_links, self.max_depth);

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
