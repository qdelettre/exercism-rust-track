pub fn build_proverb(list: &[&str]) -> String {
    match list.len() {
        0 => String::new(),
        _ => {
            let mut proverb =
                list.iter()
                    .enumerate()
                    .fold(String::from(""), |mut s, (pos, stuff)| match pos {
                        0 => s,
                        pos => {
                            s.push_str(&format!(
                                "For want of a {} the {} was lost.\n",
                                list[pos - 1],
                                stuff
                            ));
                            s
                        }
                    });

            proverb.push_str(&format!("And all for the want of a {}.", list[0]));
            proverb
        }
    }
}
