extern crate core;

use std::ops::{Div, Rem};

pub fn is_armstrong_number(num: u32) -> bool {
    if num == 0 {
        return true;
    }

    let num_digits = num.ilog10() + 1;
    let armstrong_sum = IterableNumber(num)
        .into_iter()
        .try_fold(0u32, |sum, digit| sum.checked_add(digit.pow(num_digits)));

    match armstrong_sum {
        None => false,
        Some(x) => x == num,
    }
}

struct IterableNumber<T>(T);

impl<T> IntoIterator for IterableNumber<T>
where
    T: Copy + Clone + Div<Output = T> + Rem<Output = T> + PartialEq + From<u8>,
{
    type Item = T;
    type IntoIter = NumberIterator<T>;

    fn into_iter(self) -> Self::IntoIter {
        NumberIterator {
            number: Some(self.0),
        }
    }
}

struct NumberIterator<T> {
    number: Option<T>,
}

impl<T> Iterator for NumberIterator<T>
where
    T: Copy + Clone + Div<Output = T> + Rem<Output = T> + PartialEq + From<u8>,
{
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(current_num) = self.number {
            let res = (current_num / 10u8.into(), current_num % 10u8.into());

            if res.0 == 0u8.into() {
                self.number = None;
            } else {
                self.number = Some(res.0);
            }

            Some(res.1)
        } else {
            None
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_number_iterator() {
        let num_iter = NumberIterator { number: Some(123) };

        assert_eq!(num_iter.collect::<Vec<_>>(), [3, 2, 1]);
    }

    #[test]
    fn test_number_iterator_zero() {
        let num_iter = NumberIterator { number: Some(0) };

        assert_eq!(num_iter.collect::<Vec<_>>(), [0]);
    }

    #[test]
    fn test_iterable_number() {
        let num = IterableNumber(123);

        assert_eq!(num.into_iter().collect::<Vec<_>>(), [3, 2, 1]);
    }
}
