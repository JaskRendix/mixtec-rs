use crate::word_frequencies;
use std::path::Path;

pub fn export_vocab<P1, P2>(corpus_path: P1, output_path: P2) -> anyhow::Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let freq = word_frequencies(corpus_path)?;

    let mut items: Vec<_> = freq.into_iter().collect();

    items.sort_by_key(|b| std::cmp::Reverse(b.1));

    let mut writer = csv::Writer::from_path(output_path)?;

    writer.write_record(["word", "count"])?;

    for (word, count) in items {
        writer.write_record([word, count.to_string()])?;
    }

    writer.flush()?;

    Ok(())
}
