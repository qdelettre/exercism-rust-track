use regex::Regex;

pub fn abbreviate(phrase: &str) -> String {
    if phrase.is_empty() {
        return String::new();
    }

    let pattern = Regex::new(r"([A-Za-z][a-z^']+|[a-zA-Z^']+)").unwrap();
    pattern
        .find_iter(phrase)
        .map(|matched| match matched.as_str().to_uppercase().get(0..1) {
            Some(v) => v.to_string(),
            _ => String::new(),
        })
        .collect()
}
