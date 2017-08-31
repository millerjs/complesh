use ::completer::Completer;
use ::ring_buffer::RingBuffer;
use ::filter::{Filter, WeightedMatch};

pub struct ListCompleter {
    choices: Vec<String>
}

impl ListCompleter {
    pub fn new(choices: Vec<String>) -> Self {
        ListCompleter { choices }
    }
}

impl Completer for ListCompleter {
    fn label(&self) -> String {
        "list".to_string()
    }

    fn complete<F: Filter>(&mut self, query: &str) -> RingBuffer<String> {
        let mut completions: Vec<_> = self.choices.iter()
            .filter_map(|p| F::matched(query, &*p))
            .collect();

        completions.sort_by(WeightedMatch::cmp);
        let completions = completions.into_iter().map(|m| m.result).collect();
        RingBuffer::from_vec(completions)
    }
}
