use csv::Reader;
use serde::Deserialize;
use std::path::Path;

pub mod concordance;
pub mod contexts;
pub mod export;
pub mod export_kwic;
pub mod export_vocab;
pub mod frequency;
pub mod kwic;
pub mod top;
pub mod vocabulary;

pub use concordance::*;
pub use contexts::*;
pub use export::*;
pub use export_kwic::*;
pub use export_vocab::*;
pub use frequency::*;
pub use kwic::*;
pub use top::*;
pub use vocabulary::*;

#[derive(Debug, Clone, Deserialize)]
pub struct SentencePair {
    pub source_sentence: String,
    pub target_sentence: String,
}

pub fn load_csv<P: AsRef<Path>>(path: P) -> anyhow::Result<Vec<SentencePair>> {
    let mut rdr = Reader::from_path(path)?;

    let mut pairs = Vec::new();

    for result in rdr.deserialize() {
        let pair: SentencePair = result?;
        pairs.push(pair);
    }

    Ok(pairs)
}

pub struct CorpusStats {
    pub pairs: usize,
    pub source_words: usize,
    pub target_words: usize,
}

pub fn stats<P: AsRef<Path>>(path: P) -> anyhow::Result<CorpusStats> {
    let pairs = load_csv(path)?;

    let source_words = pairs
        .iter()
        .map(|p| p.source_sentence.split_whitespace().count())
        .sum();

    let target_words = pairs
        .iter()
        .map(|p| p.target_sentence.split_whitespace().count())
        .sum();

    Ok(CorpusStats {
        pairs: pairs.len(),
        source_words,
        target_words,
    })
}
