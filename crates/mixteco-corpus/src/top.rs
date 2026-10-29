use crate::word_frequencies;
use std::path::Path;

pub fn top_words<P: AsRef<Path>>(path: P, limit: usize) -> anyhow::Result<Vec<(String, usize)>> {
    let freq = word_frequencies(path)?;

    let mut items: Vec<_> = freq.into_iter().collect();

    items.sort_by_key(|b| std::cmp::Reverse(b.1));

    items.truncate(limit);

    Ok(items)
}
