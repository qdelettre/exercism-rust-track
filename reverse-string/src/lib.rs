use unicode_segmentation::UnicodeSegmentation;

pub fn reverse(input: &str) -> String {
    // w/o grapheme clusters
    // input.chars().rev().collect()

    // w/ grapheme clusters
    input.graphemes(true).rev().collect()
}
