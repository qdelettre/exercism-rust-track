// https://github.com/wackywendell/primes/blob/master/src/lib.rs

fn firstfac(x: u64) -> u64 {
    if x % 2 == 0 {
        return 2;
    };
    for n in (1..).map(|m| 2 * m + 1).take_while(|m| m * m <= x) {
        if x % n == 0 {
            return n;
        };
    }
    x
}

pub fn factors(n: u64) -> Vec<u64> {
    if n <= 1 {
        return vec![];
    };
    let mut factors: Vec<u64> = Vec::new();
    let mut current_number = n;
    loop {
        let m = firstfac(current_number);
        factors.push(m);
        if m == current_number {
            break;
        } else {
            current_number /= m
        };
    }
    factors
}
