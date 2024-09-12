use std::ops::Rem;

pub fn is_leap_year<T>(year: T) -> bool
where
    T: DivisibleBy + Copy + From<i16>
{
    if year.divisible_by(100i16.into()) {
        return year.divisible_by(400i16.into());
    }

    year.divisible_by(4i16.into())
}

pub trait DivisibleBy {
    fn divisible_by(self, n: Self) -> bool;
}

impl<T> DivisibleBy for T
where
    T: Rem<Output = T> + PartialEq + From<u8>,
{
    fn divisible_by(self, n: Self) -> bool {
        self % n == 0u8.into()
    }
}
