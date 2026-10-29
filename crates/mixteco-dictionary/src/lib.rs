use ahash::AHashMap;
use mixteco_core::tokenize;
use mixteco_corpus::{SentencePair, load_csv};
use mixteco_search::search_form;

#[derive(Debug, Clone)]
pub struct Lexeme {
    pub lemma: String,
    pub frequency: usize,
}

#[derive(Debug, Clone)]
pub struct TranslationCandidate {
    pub spanish: String,
    pub score: f64,
    pub cooccurrences: usize,
    pub global_frequency: usize,
}

#[derive(Debug, Clone)]
pub struct DictionaryEntry {
    pub lemma: String,
    pub frequency: usize,
    pub translations: Vec<TranslationCandidate>,
}

const STOPWORDS: &[&str] = &[
    "a", "al", "como", "con", "de", "del", "el", "ella", "ellas", "ellos", "en", "es", "la", "las",
    "lo", "los", "no", "para", "pero", "por", "que", "se", "su", "sus", "un", "una", "y", "él",
    "le", "les", "mi", "si", "sino", "ha", "son", "está", "fue", "dijo", "cuando", "entonces",
    "todos", "todo", "cosas", "ustedes", "me", "nos", "más", "muy", "ya", "así", "ahora", "este",
    "esta", "estos", "estas", "ese", "esa", "eso",
];

const MIXTEC_STOPWORDS: &[&str] = &["ni", "ja", "ka", "cha", "chi", "ra", "ya", "na", "de", "a"];

fn normalize_spanish_word(word: &str) -> String {
    word.trim_matches(|c: char| !c.is_alphanumeric())
        .to_lowercase()
}

pub fn spanish_word_frequencies(path: &str) -> anyhow::Result<AHashMap<String, usize>> {
    let pairs = load_csv(path)?;

    let mut freq = AHashMap::new();

    for pair in pairs {
        for word in pair.target_sentence.split_whitespace() {
            let word = normalize_spanish_word(word);

            if word.is_empty() {
                continue;
            }

            *freq.entry(word).or_insert(0) += 1;
        }
    }

    Ok(freq)
}

pub fn mixtec_frequencies(path: &str) -> anyhow::Result<AHashMap<String, usize>> {
    let pairs = load_csv(path)?;

    let mut freq = AHashMap::new();

    for pair in pairs {
        for token in tokenize(&pair.source_sentence) {
            let token = search_form(&token);

            if token.is_empty() {
                continue;
            }

            *freq.entry(token).or_insert(0) += 1;
        }
    }

    Ok(freq)
}

fn translation_candidates_from_data(
    pairs: &[SentencePair],
    global_freq: &AHashMap<String, usize>,
    mixtec_word: &str,
) -> Vec<TranslationCandidate> {
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
        .filter_map(|(spanish, cooccurrences)| {
            let global = *global_freq.get(&spanish).unwrap_or(&1);

            if cooccurrences < 3 {
                return None;
            }

            if global < 3 {
                return None;
            }

            Some(TranslationCandidate {
                spanish,
                cooccurrences,
                global_frequency: global,
                score: cooccurrences as f64 / (global as f64).sqrt(),
            })
        })
        .collect();

    candidates.sort_by(|a, b| {
        b.score
            .partial_cmp(&a.score)
            .unwrap_or(std::cmp::Ordering::Equal)
    });

    candidates
}

pub fn translation_candidates(
    path: &str,
    mixtec_word: &str,
) -> anyhow::Result<Vec<TranslationCandidate>> {
    let pairs = load_csv(path)?;
    let global_freq = spanish_word_frequencies(path)?;

    Ok(translation_candidates_from_data(
        &pairs,
        &global_freq,
        mixtec_word,
    ))
}

pub fn dictionary(path: &str, min_frequency: usize) -> anyhow::Result<Vec<DictionaryEntry>> {
    println!("Loading corpus...");

    let pairs = load_csv(path)?;
    let global_freq = spanish_word_frequencies(path)?;
    let freqs = mixtec_frequencies(path)?;

    println!("Corpus loaded.");

    let mut entries = Vec::new();
    let mut processed = 0usize;

    for (lemma, frequency) in freqs {
        if MIXTEC_STOPWORDS.contains(&lemma.as_str()) {
            continue;
        }

        if frequency < min_frequency {
            continue;
        }

        processed += 1;

        println!("[{}] Processing {} ({})", processed, lemma, frequency);

        let translations = translation_candidates_from_data(&pairs, &global_freq, &lemma);

        entries.push(DictionaryEntry {
            lemma,
            frequency,
            translations: translations.into_iter().take(10).collect(),
        });
    }

    entries.sort_by_key(|e| std::cmp::Reverse(e.frequency));

    println!("Finished. Generated {} entries.", entries.len());

    Ok(entries)
}
