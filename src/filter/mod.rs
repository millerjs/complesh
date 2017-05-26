pub struct WeightedMatch {
    pub weight: f32,
    pub result: String,
}

pub trait Filter {
    fn matched(query: &str, value: &str) -> Option<WeightedMatch>;
}

mod spaced;

pub use self::spaced::SpacedFilter;
