use walkdir::WalkDir;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::env::home_dir;
use ::completer::{Completer, emphasize};
use ::ring_buffer::RingBuffer;


pub struct RecursiveCompleter {
    cache: HashMap<String, Vec<String>>,
    root: String,
    max_depth: usize,
}


struct WeightedMatch {
    weight: f32,
    result: String,
}

fn expand_user(path: &str) -> String {
    if path.starts_with("~/") {
        if let Some(home) = home_dir() {
            home.display().to_string() + &path[2..]
        } else {
            path.to_string()
        }
    } else {
        path.to_string()
    }
}


fn spaced_match_highlight(query: &str, value: &str) -> Option<WeightedMatch> {
    let mut query = expand_user(query).to_lowercase().chars().rev().collect::<String>();
    let mut result = String::with_capacity(value.len());

    let mut c_query_opt = query.pop();
    let mut run = 0;
    let mut weight = 0.0;

    for c_value in value.to_string().to_lowercase().chars() {
        if let Some(c_query) = c_query_opt {
            if c_query == c_value {
                result += &*emphasize(c_value);
                c_query_opt = query.pop();
                if run > 0 {
                    weight += 1.0;
                }
                run += 1;
            } else {
                run = 0;
                result.push(c_value);
            }
        } else {
            run = 0;
            result.push(c_value);
        }
    }

    if result.starts_with("./") {
        result = result[2..].to_string();
    }

    if query.is_empty() {
        let weight = weight / (value.len() as f32);
        Some(WeightedMatch { result, weight })
    } else {
        None
    }
}

impl RecursiveCompleter {
    pub fn new() -> Self {
        RecursiveCompleter {
            cache: HashMap::new(),
            root: ".".to_string(),
            max_depth: 5,
        }
    }

    fn update_root(&mut self, query: &str) {
        if query.starts_with("~/") {
            if let Some(path) = home_dir() {
                self.root = path.display().to_string()
            } else {
                self.root = ".".to_string()
            }
        }
        else if query.starts_with("/") {
            self.root = "/".to_string()
        } else {
            self.root = ".".to_string()
        }
    }

    fn cache<'a>(&'a mut self) -> &'a Vec<String>{
        if !self.cache.contains_key(&self.root) {
            let root = &self.root;
            let paths = WalkDir::new(&*root)
                .max_depth(self.max_depth)
                .into_iter()
                .filter_map(|e| e.ok())
                .map(|e| e.path().display().to_string())
                .collect();
            self.cache.insert(self.root.clone(), paths);
        }
        &self.cache[&self.root]
    }
}

impl Completer for RecursiveCompleter {
    fn complete(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        self.update_root(query);
        let mut completions: Vec<_> = self.cache().iter()
            .filter_map(|p| spaced_match_highlight(query, p))
            .collect();
        completions.sort_by(|a, b| b.weight.partial_cmp(&a.weight).unwrap_or(Ordering::Equal));
        RingBuffer::from_vec(
            completions.into_iter()
                .map(|comp| comp.result)
                .take(limit)
                .collect())
    }
}
