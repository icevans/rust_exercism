use rand::prelude::*;

pub fn private_key(p: u64) -> u64 {
    let mut rng = thread_rng();
    let x: u64 = rng.gen_range(2..p);

    x
}

pub fn public_key(p: u64, g: u64, private_key: u64) -> u64 {
    // public_key = g^private_key mod p
    let mut key = 1u64;

    for _ in 0..private_key {
        key = (key.wrapping_mul(g)) % p;
    }

    key
}

pub fn secret(p: u64, other_public_key: u64, private_key: u64) -> u64 {
    let mut secret = 1u64;

    for _ in 0..private_key {
        secret = (secret.wrapping_mul(other_public_key)) % p;
    }

    secret
}
