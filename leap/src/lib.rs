pub fn is_leap_year(year: u64) -> bool {
    if year.divisible_by(100) {
        return year.divisible_by(400);
    }

    year.divisible_by(4)
}

trait DivisibleBy {
    fn divisible_by(self, n: Self) -> bool;
}

impl DivisibleBy for u64 {
    fn divisible_by(self, n: Self) -> bool {
        self % n == 0
    }
}
