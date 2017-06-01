use std::cmp::Ordering;

pub struct WeightedMatch {
    pub weight: f32,
    pub result: String,
    pub original: String,
}

pub trait Filter {
    fn matched(query: &str, value: &str) -> Option<WeightedMatch>;
}

impl WeightedMatch {
    pub fn cmp(&self, other: &WeightedMatch) -> Ordering {
        other.weight.partial_cmp(&self.weight).unwrap_or(Ordering::Equal)
    }
}

mod spaced;

pub use self::spaced::SpacedFilter;
