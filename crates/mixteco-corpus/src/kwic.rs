use crate::load_csv;
use mixteco_core::tokenize;
use mixteco_search::search_form;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct KwicHit {
    pub left: String,
    pub keyword: String,
    pub right: String,
    pub translation: String,
}

pub fn kwic<P: AsRef<Path>>(path: P, query: &str, width: usize) -> anyhow::Result<Vec<KwicHit>> {
    let pairs = load_csv(path)?;

    let normalized_query = search_form(query);

    let mut results = Vec::new();

    for pair in pairs {
        let tokens = tokenize(&pair.source_sentence);

        for (i, token) in tokens.iter().enumerate() {
            if search_form(token) == normalized_query {
                let left_start = i.saturating_sub(width);

                let left = tokens[left_start..i].join(" ");

                let right_end = (i + 1 + width).min(tokens.len());

                let right = if i + 1 < right_end {
                    tokens[i + 1..right_end].join(" ")
                } else {
                    String::new()
                };

                results.push(KwicHit {
                    left,
                    keyword: token.clone(),
                    right,
                    translation: pair.target_sentence.clone(),
                });
            }
        }
    }

    Ok(results)
}
