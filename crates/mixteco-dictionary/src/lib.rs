use ahash::AHashMap;
use mixteco_core::tokenize;
use mixteco_corpus::load_csv;
use mixteco_search::search_form;

#[derive(Debug, Clone)]
pub struct Lexeme {
    pub lemma: String,
    pub frequency: usize,
}

#[derive(Debug, Clone)]
pub struct TranslationCandidate {
    pub spanish: String,
    pub score: usize,
}

const STOPWORDS: &[&str] = &[
    "a", "al", "como", "con", "de", "del", "el", "ella", "ellas", "ellos", "en", "es", "la", "las",
    "lo", "los", "no", "para", "pero", "por", "que", "se", "su", "sus", "un", "una", "y", "él",
    "le", "les", "mi", "si", "sino", "ha", "son", "está", "fue", "dijo", "cuando", "entonces",
    "todos", "todo", "cosas", "ustedes", "me", "nos", "más", "muy", "ya", "así", "ahora", "este",
    "esta", "estos", "estas", "ese", "esa", "eso",
];

fn normalize_spanish_word(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

pub fn translation_candidates(
    path: &str,
    mixtec_word: &str,
) -> anyhow::Result<Vec<TranslationCandidate>> {
    let pairs = load_csv(path)?;

    let query = search_form(mixtec_word);

    let mut counts = AHashMap::new();

    for pair in pairs {
        let contains_word = tokenize(&pair.source_sentence)
            .iter()
            .any(|token| search_form(token) == query);

        if !contains_word {
            continue;
        }

        for word in pair.target_sentence.split_whitespace() {
            let word = normalize_spanish_word(word);

            if word.is_empty() {
                continue;
            }

            if STOPWORDS.contains(&word.as_str()) {
                continue;
            }

            *counts.entry(word).or_insert(0usize) += 1;
        }
    }

    let mut candidates: Vec<_> = counts
        .into_iter()
        .map(|(spanish, score)| TranslationCandidate { spanish, score })
        .collect();

    candidates.sort_by_key(|candidate| std::cmp::Reverse(candidate.score));

    Ok(candidates)
}
