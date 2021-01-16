// see https://stackoverflow.com/questions/9625663/calculating-and-printing-the-nth-prime-number/9704912#9704912

fn is_prime(n: u32) -> bool {
    if n % 2 == 0 {
        return n == 2;
    }
    if n % 3 == 0 {
        return n == 3;
    }
    let mut step = 4;
    let m = ((n as f64).sqrt() as u32) + 1;
    for i in 5..m {
        if n % i == 0 {
            return false;
        }
        step = 6 - step;
    }
    return true;
}

pub fn nth(n: u32) -> u32 {
    let mut candidate = 3;
    let mut count = 0;
    while count < n {
        if is_prime(candidate) {
            count = count + 1;
        }
        candidate = candidate + 1;
    }
    candidate - 1
}
