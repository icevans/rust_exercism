use rand::prelude::*;

pub fn private_key(prime_1: u64) -> u64 {
    let mut rng = thread_rng();
    rng.gen_range(2..prime_1)
}

pub fn public_key(prime_1: u64, prime_2: u64, private_key: u64) -> u64 {
    mod_pow(prime_2, private_key, prime_1)
}

pub fn secret(prime_1: u64, other_public_key: u64, private_key: u64) -> u64 {
    mod_pow(other_public_key, private_key, prime_1)
}

pub fn mod_pow(base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0;
    }

    let mut result: u128 = 1;
    let mut new_base: u128 = (base % modulus) as u128;

    while exp > 0 {
        if exp % 2 == 1 {
            result = (result * new_base) % modulus as u128;
        }

        new_base = (new_base * new_base) % modulus as u128;
        exp >>= 1;
    }

    result.try_into().expect("this will always fit")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_mod_pow() {
        assert_eq!(mod_pow(4, 13, 497), 445);
    }
}
