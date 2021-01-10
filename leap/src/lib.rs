pub fn is_leap_year(year: u64) -> bool {
    match year % 4 {
        0 => match year % 100 {
            0 => year % 400 == 0,
            _ => true
        },
        _ => false
    }
}
