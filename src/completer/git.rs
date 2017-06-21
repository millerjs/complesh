use ::completer::{Completer, CompleterBase};
use ::filter::Filter;
use ::ring_buffer::RingBuffer;
use ::util::git_root;
use crossbeam::sync::MsQueue;
use ignore::WalkState::Continue;
use ignore::{WalkBuilder, DirEntry};
use std::env::home_dir;
use std::sync::Arc;

pub struct GitCompleter {
    base: CompleterBase,
    pub max_depth: usize,
    pub root: String,
}

fn walk_dir_ignore(path: &str, max_depth: usize) -> Vec<String> {
    let queue: Arc<MsQueue<Option<DirEntry>>> = Arc::new(MsQueue::new());
    let stdout_queue = queue.clone();

    let walker = WalkBuilder::new(path).threads(8).max_depth(Some(max_depth)).build_parallel();
    walker.run(|| {
        let queue = queue.clone();
        Box::new(move |result| { if let Ok(res) = result {queue.push(Some(res))}; Continue })
    });
    queue.push(None);

    let mut paths = vec![];
    while let Some(dent) = stdout_queue.pop() {
        paths.push(dent.path().to_string_lossy().to_string())
    }
    paths
}

impl GitCompleter {
    pub fn new() -> Self {
        GitCompleter {
            base: CompleterBase::new(),
            root: ".".to_string(),
            max_depth: 32,
        }
    }

    fn update_root(&mut self, query: &str) {
        if query.starts_with("~/") {
            if let Some(path) = home_dir() {
                self.root = path.display().to_string();
            } else {
                self.root = ".".to_string();
            }
        } else if query.starts_with("/") {
            self.root = "/".to_string();
        } else if query.starts_with("./") {
            self.root = ".".to_string()
        } else if let Ok(root) = git_root(".") {
            self.root = root;
        } else {
            self.root = ".".to_string();
        }
    }
}

impl Completer for GitCompleter {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        self.update_root(query);
        let root = self.root.clone() + "/";
        let depth = self.max_depth;
        self.base.complete::<F, _>(query, &*root, limit, || { walk_dir_ignore(&*root, depth) })
    }
}
