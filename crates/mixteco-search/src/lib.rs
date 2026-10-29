use mixteco_orthography::normalize;

pub fn search_form(input: &str) -> String {
    normalize(input).replace('ꞌ', "")
}
