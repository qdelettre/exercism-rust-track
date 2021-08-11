use std::collections::BTreeMap;

pub fn transform(h: &BTreeMap<i32, Vec<char>>) -> BTreeMap<char, i32> {
    h.iter()
        .flat_map(|(&n, vec_data)| vec_data.iter().map(move |s| (s.to_ascii_lowercase(), n)))
        .collect()
}
