pub fn normalize(input: &str) -> String {
    input.to_lowercase().replace(['\'', '’'], "ꞌ")
}
