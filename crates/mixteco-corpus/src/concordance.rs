use crate::load_csv;
use mixteco_core::tokenize;
use mixteco_search::search_form;
use std::path::Path;

#[derive(Debug, Clone)]
pub struct ConcordanceHit {
    pub source: String,
    pub target: String,
}

pub fn concordance<P: AsRef<Path>>(path: P, query: &str) -> anyhow::Result<Vec<ConcordanceHit>> {
    let pairs = load_csv(path)?;

    let normalized_query = search_form(query);

    let mut results = Vec::new();

    for pair in pairs {
        let tokens = tokenize(&pair.source_sentence);

        let found = tokens
            .iter()
            .any(|token| search_form(token) == normalized_query);

        if found {
            results.push(ConcordanceHit {
                source: pair.source_sentence,
                target: pair.target_sentence,
            });
        }
    }

    Ok(results)
}
