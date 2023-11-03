fn luhn_double(x: u32) -> u32 {
    let y = x * 2;

    if y > 9 {
        return y - 9;
    }

    y
}

struct LuhnResult {
    num_digits: u32,
    luhn_sum: u32,
}

/// Check a Luhn checksum
pub fn is_valid(code: &str) -> bool {
    let mut should_double = false;

    let result = LuhnResult {
        num_digits: 0,
        luhn_sum: 0,
    };

    let luhn_sum = code
        .chars()
        .filter(|character| character != &' ')
        .map(|character| character.to_digit(10))
        .rev()
        .try_fold(result, |result, digit| {
            if digit.is_none() {
                return None;
            }

            let mut digit = digit.unwrap();

            if should_double {
                digit = luhn_double(digit);
            }

            should_double = !should_double;

            Some(LuhnResult {
                num_digits: result.num_digits + 1,
                luhn_sum: result.luhn_sum + digit,
            })
        });

    match luhn_sum {
        Some(LuhnResult {
            num_digits: 0..=1,
            luhn_sum: _,
        }) => false,
        Some(LuhnResult {
            num_digits: _,
            luhn_sum: sum,
        }) => sum % 10 == 0,
        None => false,
    }
}
