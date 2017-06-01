use std::collections::HashMap;
use std::env::home_dir;
use ::completer::Completer;
use ::ring_buffer::RingBuffer;
use ::filter::{Filter, WeightedMatch};
use ::util::git_root;
use ignore;
use std::sync::Arc;
use std::env::current_dir;

use crossbeam::sync::MsQueue;
use ignore::WalkBuilder;
use ignore::WalkState::Continue;

pub struct GitCompleter {
    cache: HashMap<String, Vec<String>>,
    root: String,
    max_depth: usize,
    cwd: String,
}

fn walk_dir_ignore(path: &str, max_depth: usize) -> Vec<String> {
    let queue: Arc<MsQueue<Option<ignore::DirEntry>>> = Arc::new(MsQueue::new());
    let stdout_queue = queue.clone();

    let walker = WalkBuilder::new(path).threads(8).max_depth(Some(max_depth)).build_parallel();
    walker.run(|| {
        let queue = queue.clone();
        Box::new(move |result| { queue.push(Some(result.unwrap())); Continue })
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
            cache: HashMap::new(),
            root: ".".to_string(),
            max_depth: 5,
            cwd: current_dir().unwrap().display().to_string(),
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
        } else if let Some(root) = git_root() {
            self.root = root;
        } else {
            self.root = ".".to_string();
        }
    }

    fn cache<'a>(&'a mut self) -> &'a Vec<String>{
        if !self.cache.contains_key(&self.root) {
            let paths = walk_dir_ignore(&*self.root, self.max_depth);
            self.cache.insert(self.root.clone(), paths);
        }
        &self.cache[&self.root]
    }
}

impl Completer for GitCompleter {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        self.update_root(query);
        let root = self.root.clone() + "/";

        let mut completions: Vec<_> = self.cache().iter()
            .map(|p| p.replace(&*root, ""))
            .filter_map(|p| F::matched(query, &*p))
            .collect();

        completions.sort_by(WeightedMatch::cmp);
        RingBuffer::from_vec(
            completions.into_iter()
                .map(|comp| format!("{}/{}", self.root, comp.result))
                .take(limit)
                .collect())
    }
}
