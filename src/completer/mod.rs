use glob::glob;
use std::fmt::Display;


pub trait Completer {
    fn complete<S>(beginning: S, limit: usize) -> Vec<String> where S: Into<String>;
}

pub struct DefaultCompleter;
impl Completer for DefaultCompleter {
    fn complete<S>(beginning: S, _: usize) -> Vec<String> where S: Into<String> {
        vec![beginning.into()]
    }
}

pub struct FileCompleter;
impl Completer for FileCompleter {
    fn complete<S>(beginning: S, limit: usize) -> Vec<String> where S: Into<String> {
        let completions = glob(format!("{}*", beginning.into()).as_str()).unwrap()
            .map(|path| format!("{}", path.unwrap().as_path().to_str().unwrap()))
            .take(limit)
            .collect::<Vec<_>>();
        vec![]
    }
}
