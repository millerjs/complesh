use ::completer::{Completer, CompleterBase};
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use ::util::search_root;
use walkdir::WalkDir;

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

impl Completer for RecursiveCompleter {
    fn label(&self) -> String {
        "recursive".to_string()
    }

    fn complete<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        let root = search_root(query).to_string_lossy().to_string();
        let (links, depth) = (self.follow_links, self.max_depth);

        self.base.complete::<F, _>(query, &*root, || {
            WalkDir::new(&root)
                .follow_links(links)
                .max_depth(depth)
                .into_iter()
                .map(|p| p.unwrap().path().to_str().unwrap().to_string())
                .collect()
        })
    }
}
