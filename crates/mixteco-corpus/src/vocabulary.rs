use crate::word_frequencies;
use std::path::Path;

#[derive(Debug)]
pub struct VocabularyStats {
    pub unique_words: usize,
    pub hapax_words: usize,
    pub hapax_rate: f64,

    pub most_common_word: String,
    pub most_common_count: usize,
}

pub fn vocabulary_stats<P: AsRef<Path>>(path: P) -> anyhow::Result<VocabularyStats> {
    let freq = word_frequencies(path)?;

    let unique_words = freq.len();

    let hapax_words = freq.values().filter(|&&count| count == 1).count();

    let hapax_rate = if unique_words > 0 {
        (hapax_words as f64 / unique_words as f64) * 100.0
    } else {
        0.0
    };

    let (most_common_word, most_common_count) = freq
        .iter()
        .max_by_key(|(_, count)| *count)
        .map(|(word, count)| (word.clone(), *count))
        .unwrap_or_default();

    Ok(VocabularyStats {
        unique_words,
        hapax_words,
        hapax_rate,
        most_common_word,
        most_common_count,
    })
}
