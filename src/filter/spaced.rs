use ::util::{emphasize, expand_user};
use ::filter::{WeightedMatch, Filter};
use nlp_tokenize::{WhitePunctTokenizer, Tokenizer};
use std::cmp::max;

lazy_static! {
    static ref TOKENIZER: WhitePunctTokenizer = WhitePunctTokenizer::new();
}

pub enum SpacedFilter {}

impl SpacedFilter {
    pub fn weigh(query: &str, value: &str) -> Option<WeightedMatch> {
        let original = value.to_string();
        let mut query = expand_user(query).chars().rev().collect::<String>();
        let mut result = String::with_capacity(value.len());

        let mut c_query_opt = query.pop();
        let mut run = true;
        let mut weight = 0.0;
        let mut first_char = None;

        for (i, c_value) in value.to_string().chars().enumerate() {
            let c_value_lower: String = c_value.to_lowercase().collect();
            if let Some(c_query) = c_query_opt {
                let c_query_lower: String = c_query.to_lowercase().collect();
                if c_query_lower == c_value_lower {
                    result += &*emphasize(c_value);
                    c_query_opt = query.pop();
                    weight += if run { 4.0 } else { 1.0 };
                    if first_char.is_none() { first_char = Some(i); }
                    run = true;
                } else {
                    run = false;
                    result.push(c_value);
                }
            } else {
                run = false;
                result.push(c_value);
            }
        }

        if result.starts_with("./") {
            result = result[2..].to_string();
        }

        if query.is_empty() {
            let mut weight = weight / (value.len() as f32).sqrt();
            if let Some(idx) = first_char { weight *= max(1, idx) as f32 }
            Some(WeightedMatch { result, weight, original })
        } else {
            None
        }
    }

    fn offset_match(query: &str, value: &str, offset: usize) -> Option<WeightedMatch> {
        SpacedFilter::weigh(query, &value[offset..])
            .map(|m| WeightedMatch { result: format!("{}{}", &value[..offset], m.result), ..m })
    }
}


impl Filter for SpacedFilter {
    fn matched(query: &str, value: &str) -> Option<WeightedMatch> {
        let first_match = SpacedFilter::weigh(query, value);
        let mut matches = match first_match {
            None => return None,
            Some(m) => vec![m],
        };

        let tokens = TOKENIZER.tokenize(value);
        let token_offset_results = tokens.into_iter()
            .filter_map(|token| SpacedFilter::offset_match(query, value, token.0))
            .collect::<Vec<_>>();

        matches.extend(token_offset_results);
        matches.sort_by(WeightedMatch::cmp);
        matches.into_iter().nth(0)
    }
}
