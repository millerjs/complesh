use ::filter::{Filter, WeightedMatch};
use ::ring_buffer::RingBuffer;
use rayon::prelude::*;
use std::collections::HashMap;

pub trait Completer {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String>;

    fn toggle_mode(&mut self) {}
    fn label(&self) -> String;
}

mod recursive;
mod git;
mod mixed;
mod list;

pub use self::git::GitCompleter;
pub use self::recursive::RecursiveCompleter;
pub use self::list::ListCompleter;
pub use self::mixed::MixedCompleter;


pub struct CompleterBase {
    cache: HashMap<String, Vec<String>>
}


impl CompleterBase {
    fn new() -> CompleterBase {
        CompleterBase { cache: HashMap::new() }
    }

    fn cache<'a, F>(&'a mut self, root: &str, f: F) -> &'a Vec<String>
        where F: FnOnce() -> Vec<String>
    {
        if !self.cache.contains_key(root) {
            let paths = f();
            self.cache.insert(root.to_string(), paths);
        }
        &self.cache[root]
    }

    pub fn complete<F, G>(&mut self, query: &str, root: &str, limit: usize, completer: G)
                          -> RingBuffer<String>
        where G: FnOnce() -> Vec<String>, F: Filter
    {
        let mut completions: Vec<_> = self.cache(&*root, completer).par_iter()
            .map(|p| p.replace("./", ""))
            .filter_map(|p| F::matched(query, &*p))
            .collect();

        completions.sort_by(WeightedMatch::cmp);

        let results = completions.into_iter()
            .map(|comp| comp.result.to_string())
            .take(limit)
            .collect();

        RingBuffer::from_vec(results)
    }
}
