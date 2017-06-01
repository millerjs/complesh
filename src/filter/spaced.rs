use ::util::{emphasize, expand_user, find_all};
use ::filter::{WeightedMatch, Filter};

pub enum SpacedFilter {}

impl SpacedFilter {
    pub fn test(query: &str, value: &str) -> Option<WeightedMatch> {
        let original = value.to_string();
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
            let weight = weight / (value.len() as f32).sqrt();
            Some(WeightedMatch { result, weight, original })
        } else {
            None
        }
    }

    fn offset_match(query: &str, value: &str, offset: usize) -> Option<WeightedMatch> {
        SpacedFilter::test(query, &value[offset..])
            .map(|m| WeightedMatch { result: format!("{}{}", &value[..offset], m.result), ..m })
    }
}


impl Filter for SpacedFilter {
    fn matched(query: &str, value: &str) -> Option<WeightedMatch> {
        let first_match = SpacedFilter::test(query, value);
        let mut matches = match first_match {
            None => return None,
            Some(m) => vec![m],
        };

        matches.extend(find_all(value, "/").into_iter()
                       .filter_map(|index| SpacedFilter::offset_match(query, value, index))
                       .collect::<Vec<_>>());

        matches.sort_by(WeightedMatch::cmp);
        matches.into_iter().nth(0)
    }
}
