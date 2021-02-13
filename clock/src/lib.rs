use std::fmt::Display;

const DAY: i32 = 1440;
const HOUR: i32 = 60;

#[derive(Debug, Eq, PartialEq)]
pub struct Clock(i32);

impl Clock {
    pub fn new(h: i32, m: i32) -> Self {
        Clock(Self::normalize(h * HOUR + m))
    }

    fn normalize(minutes: i32) -> i32 {
        ((minutes % DAY) + DAY) % DAY
    }

    pub fn hours(&self) -> i32 {
        self.0 / HOUR
    }

    pub fn minutes(&self) -> i32 {
        self.0 % HOUR
    }
    pub fn add_minutes(mut self, m: i32) -> Self {
        self.0 = Self::normalize(self.0 + m);
        self
    }
}

impl Display for Clock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:02}:{:02}", self.hours(), self.minutes())
    }
}
