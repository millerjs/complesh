use ::util::{emphasize, expand_user};
use ::filter::{WeightedMatch, Filter};

pub enum SpacedFilter {}

impl Filter for SpacedFilter {
    fn matched(query: &str, value: &str) -> Option<WeightedMatch> {
        let mut query = expand_user(query).chars().rev().collect::<String>();
        let mut result = String::with_capacity(value.len());

        let mut c_query_opt = query.pop();
        let mut run = 0;
        let mut weight = 0.0;

        for c_value in value.to_string().chars() {
            let c_value_lower: String = c_value.to_lowercase().collect();
            if let Some(c_query) = c_query_opt {
                let c_query_lower: String = c_query.to_lowercase().collect();
                if c_query_lower == c_value_lower {
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

}
