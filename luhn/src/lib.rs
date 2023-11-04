fn luhn_double(x: u32) -> u32 {
    let doubled = x * 2;
    if doubled > 9 {
        return doubled - 9;
    }

    doubled
}

struct LuhnResult {
    num_digits: u32,
    luhn_sum: u32,
    double_next: bool,
}

/// Check a Luhn checksum
pub fn is_valid(code: &str) -> bool {
    let result = code
        .chars()
        .filter(|character| character != &' ')
        .map(|character| character.to_digit(10))
        .rev()
        .try_fold(
            LuhnResult {
                num_digits: 0,
                luhn_sum: 0,
                double_next: false,
            },
            |result, digit| {
                if digit.is_none() {
                    return None;
                }

                let mut digit = digit.unwrap();

                if result.double_next {
                    digit = luhn_double(digit);
                }

                Some(LuhnResult {
                    num_digits: result.num_digits + 1,
                    luhn_sum: result.luhn_sum + digit,
                    double_next: !result.double_next,
                })
            },
        );

    if let Some(result) = result {
        match result.num_digits {
            0 | 1 => false,
            _ => result.luhn_sum % 10 == 0,
        }
    } else {
        false
    }
}
