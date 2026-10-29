use mixteco_core::LanguageProfile;

pub struct XtnProfile;

impl LanguageProfile for XtnProfile {
    fn normalize(&self, text: &str) -> String {
        mixteco_orthography::normalize(text)
    }

    fn search_form(&self, text: &str) -> String {
        text.to_lowercase().replace('ꞌ', "")
    }

    fn ipa(&self, text: &str) -> String {
        mixteco_phonology::to_ipa(text)
    }

    fn tokenize(&self, text: &str) -> Vec<String> {
        text.split_whitespace().map(String::from).collect()
    }
}
