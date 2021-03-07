pub struct PascalsTriangle {
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        (0..self.row_count)
            .map(|line| {
                (0..line + 1)
                    .map(|i| PascalsTriangle::binom_knuth(line, i))
                    .collect()
            })
            .collect()
    }

    fn binom_knuth(n: u32, k: u32) -> u32 {
        (0..n + 1).rev().zip(1..k + 1).fold(1, |mut r, (n, d)| {
            r *= n;
            r /= d;
            r
        })
    }
}
