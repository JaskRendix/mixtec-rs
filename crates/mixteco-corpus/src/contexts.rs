use crate::load_csv;
use mixteco_core::tokenize;
use mixteco_search::search_form;
use std::collections::HashMap;
use std::path::Path;

#[derive(Debug)]
pub struct ContextStats {
    pub left: Vec<(String, usize)>,
    pub right: Vec<(String, usize)>,
}

pub fn contexts<P: AsRef<Path>>(path: P, query: &str) -> anyhow::Result<ContextStats> {
    let pairs = load_csv(path)?;

    let normalized_query = search_form(query);

    let mut left = HashMap::new();
    let mut right = HashMap::new();

    for pair in pairs {
        let tokens = tokenize(&pair.source_sentence);

        for (i, token) in tokens.iter().enumerate() {
            if search_form(token) == normalized_query {
                if i > 0 {
                    *left.entry(tokens[i - 1].clone()).or_insert(0) += 1;
                }

                if i + 1 < tokens.len() {
                    *right.entry(tokens[i + 1].clone()).or_insert(0) += 1;
                }
            }
        }
    }

    let mut left: Vec<_> = left.into_iter().collect();
    let mut right: Vec<_> = right.into_iter().collect();

    left.sort_by_key(|b| std::cmp::Reverse(b.1));
    right.sort_by_key(|b| std::cmp::Reverse(b.1));

    Ok(ContextStats { left, right })
}
