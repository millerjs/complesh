use ::completer::Completer;
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use glob::glob;

pub struct GlobCompleter {}

impl Completer for GlobCompleter {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        let completions = glob(format!("{}*", query).as_str()).unwrap()
            .map(|path| path.unwrap().as_path().to_str().unwrap().to_string())
            .take(limit)
            .filter_map(|path| F::matched(query, &*path))
            .map(|result| result.result)
            .collect();
        RingBuffer::from_vec(completions)
    }
}
