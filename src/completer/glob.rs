use ::completer::Completer;
use ::ring_buffer::RingBuffer;
use ::filter::Filter;
use glob::glob;

pub enum GlobCompleter {}

impl Completer for GlobCompleter {
    fn complete<F: Filter>(&mut self, query: &str, limit: usize) -> RingBuffer<String> {
        RingBuffer::from_vec(
            glob(format!("{}*", query).as_str()).unwrap()
                .map(|path| format!("{}", path.unwrap().as_path().to_str().unwrap()))
                .take(limit)
                .filter_map(|path| F::matched(query, &*path))
                .map(|result| result.result)
                .collect::<Vec<_>>())
    }
}
