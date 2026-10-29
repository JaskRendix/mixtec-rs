pub trait LanguageProfile {
    fn normalize(&self, text: &str) -> String;

    fn search_form(&self, text: &str) -> String;

    fn ipa(&self, text: &str) -> String;

    fn tokenize(&self, text: &str) -> Vec<String>;
}
