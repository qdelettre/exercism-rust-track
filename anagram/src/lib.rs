use std::collections::HashSet;

fn are_anagrams(word1: &str, word2: &str) -> bool {
    let mut chars1: Vec<char> = word1.to_lowercase().chars().collect();
    let mut chars2: Vec<char> = word2.to_lowercase().chars().collect();
    chars1.sort_unstable();
    chars2.sort_unstable();
    chars1 == chars2
}

pub fn anagrams_for<'a>(word: &str, possible_anagrams: &'a [&'a str]) -> HashSet<&'a str> {
    possible_anagrams
        .iter()
        .filter(|&possible_anagram| {
            word.to_lowercase() != possible_anagram.to_lowercase()
                && are_anagrams(word, possible_anagram)
        })
        .cloned()
        .collect()
}
