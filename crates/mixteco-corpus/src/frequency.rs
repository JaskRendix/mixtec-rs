use crate::load_csv;
use mixteco_core::tokenize;
use std::collections::HashMap;
use std::path::Path;

pub fn word_frequencies<P: AsRef<Path>>(path: P) -> anyhow::Result<HashMap<String, usize>> {
    let pairs = load_csv(path)?;

    let mut freq = HashMap::new();

    for pair in pairs {
        for word in tokenize(&pair.source_sentence) {
            *freq.entry(word.to_lowercase()).or_insert(0) += 1;
        }
    }

    Ok(freq)
}
