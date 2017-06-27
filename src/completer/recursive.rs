use ::completer::{Completer, CompleterBase};
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
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
            max_depth: 2,
            follow_links: false,
            base: CompleterBase::new(),
        }
    }
}

impl Completer for RecursiveCompleter {
    fn label(&self) -> String {
        "recursive".to_string()
    }

    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        let root = Path::new(query).parent().unwrap_or(Path::new("./")).to_str().unwrap();
        let root = if root.is_empty() { "./" } else { root };
        let (links, depth) = (self.follow_links, self.max_depth);

        self.base.complete::<F, _>(query, &*root, limit, || {
            WalkDir::new(root)
                .follow_links(links)
                .max_depth(depth)
                .into_iter()
                .map(|p| p.unwrap().path().to_str().unwrap().to_string())
                .collect()
        })
    }
}
