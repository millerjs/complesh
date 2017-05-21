use ::completer::{Completer, emphasize};
use glob::glob;

pub enum GlobCompleter {}
impl GlobCompleter {
    fn highlight_completion(&self, query: &str, completion: &str) -> String {
        let start = completion.find(query).unwrap();
        let end = start + query.len();
        format!("{}{}{}", &completion[..start], emphasize(&completion[start..end]), &completion[end..])
    }
}

impl Completer for GlobCompleter {
    fn complete(&mut self, query: &str, limit: usize) -> Vec<String> {
        glob(format!("{}*", query).as_str()).unwrap()
            .map(|path| format!("{}", path.unwrap().as_path().to_str().unwrap()))
            .take(limit)
            .map(|path| self.highlight_completion(query, &*path))
            .collect::<Vec<_>>()
    }
}
