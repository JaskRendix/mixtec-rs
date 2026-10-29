use crate::contexts;
use std::path::Path;

pub fn export_contexts<P1, P2>(corpus_path: P1, query: &str, output_path: P2) -> anyhow::Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let stats = contexts(corpus_path, query)?;

    let mut writer = csv::Writer::from_path(output_path)?;

    writer.write_record(["direction", "word", "count"])?;

    for (word, count) in stats.left {
        writer.write_record(["left", &word, &count.to_string()])?;
    }

    for (word, count) in stats.right {
        writer.write_record(["right", &word, &count.to_string()])?;
    }

    writer.flush()?;

    Ok(())
}
