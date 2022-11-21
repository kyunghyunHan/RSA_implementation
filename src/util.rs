use num_bigint::{BigInt, BigUint, RandBigInt, Sign, ToBigInt, ToBigUint};
use num_traits::{One, Zero};

pub fn gen_prime(num_bits: usize) -> BigUint {
    let mut rng = rand::thread_rng();
    let mut a = rng.gen_biguint(num_bits);

    // TODO: Better way to do this?
    while !is_prime(&a) {
        a = rng.gen_biguint(num_bits);
    }
    a
}

pub fn is_prime(num: &BigUint) -> bool {
    if num & 1_u8.to_biguint().unwrap() == 0_u8.to_biguint().unwrap() {
        return false;
    }
    // Fermat's test
    // TODO: Make the number of iterations configurable
    for _i in 0..4 {
        let mut rng = rand::thread_rng();
        let a = rng.gen_biguint_range(&0_u8.to_biguint().unwrap(), &(num - 1u32)); //check for inclusivity
        let result = a.modpow(&(num - 1u32), num);
        if result != 1_u8.to_biguint().unwrap() {
            return false;
        }
    }
    return true;
}
