use crate::kwic;
use std::path::Path;

pub fn export_kwic<P1, P2>(
    corpus_path: P1,
    query: &str,
    output_path: P2,
    width: usize,
) -> anyhow::Result<()>
where
    P1: AsRef<Path>,
    P2: AsRef<Path>,
{
    let hits = kwic(corpus_path, query, width)?;

    let mut writer = csv::Writer::from_path(output_path)?;

    writer.write_record(["left", "keyword", "right", "translation"])?;

    for hit in hits {
        writer.write_record([hit.left, hit.keyword, hit.right, hit.translation])?;
    }

    writer.flush()?;

    Ok(())
}
