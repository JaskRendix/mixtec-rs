pub fn tokenize(text: &str) -> Vec<String> {
    text.split_whitespace()
        .map(|w| {
            w.trim_matches(|c: char| c.is_ascii_punctuation() || matches!(c, '"' | '«' | '»'))
                .to_lowercase()
        })
        .filter(|w| !w.is_empty())
        .collect()
}
