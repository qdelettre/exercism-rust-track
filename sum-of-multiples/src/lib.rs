pub fn sum_of_multiples(limit: u32, factors: &[u32]) -> u32 {
    match factors.len() {
        0 => 0,
        _ => (1..limit)
            .filter(|a| factors.iter().any(|&f| f != 0 && a % f == 0))
            .sum(),
    }
}
