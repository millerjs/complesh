use ::completer::{Completer, CompleterBase};
use ::filter::Filter;
use ::ring_buffer::RingBuffer;
use ::util::{git_root, search_root, path_string};
use crossbeam::sync::MsQueue;
use ignore::WalkState::Continue;
use ignore::{WalkBuilder, DirEntry};
use std::sync::Arc;
use std::path::Path;

pub struct GitCompleter {
    base: CompleterBase,
    pub max_depth: usize,
    pub root: String,
}

fn walk_dir_ignore<P: AsRef<Path>>(root: P, max_depth: usize) -> Vec<String> {
    let queue: Arc<MsQueue<Option<DirEntry>>> = Arc::new(MsQueue::new());
    let stdout_queue = queue.clone();

    let walker = WalkBuilder::new(root.as_ref())
        .threads(8)
        .max_depth(Some(max_depth))
        .build_parallel();

    walker.run(|| {
        let queue = queue.clone();
        Box::new(move |result| { if let Ok(res) = result {queue.push(Some(res))}; Continue })
    });

    queue.push(None);

    let mut paths = vec![];
    while let Some(path) = stdout_queue.pop() {
        let relative_path = match path.path().strip_prefix(root.as_ref()) {
            Ok(relative_path) => relative_path,
            _ => path.path(),
        };
        paths.push(path_string(relative_path))
    }
    paths
}

impl Default for GitCompleter {
    fn default() -> GitCompleter {
        GitCompleter {
            base: CompleterBase::new(),
            root: String::from("."),
            max_depth: 32,
        }
    }
}

impl GitCompleter {
    fn update_root<P: AsRef<Path>>(&mut self, query: P) {
        let query_root = search_root(&query);
        self.root = match git_root(query_root) {
            Ok(root) => root,
            Err(_)   => path_string(query),
        }
    }
}

impl Completer for GitCompleter {
    fn label(&self) -> String {
        "git".to_string()
    }

    fn complete<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        self.update_root(query);
        let root = self.root.clone() + "/";

        let depth = self.max_depth;
        self.base.complete::<F, _>(&*query, &*root, || { walk_dir_ignore(&*root, depth) })
    }
}
