mod util;
//큰 숫자 관ㄹ
use num_bigint::BigUint;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::str;

/*
key
exponent:
base:
*/

pub struct Key {
    exponent: BigUint,
    base: BigUint,
}

impl Key {
    pub fn to_string(&self) -> String {
        format!(
            "{} {}",
            //to_str_radix:주어진 기수에서 문자열 형식의 정수를 반환
            self.exponent.to_str_radix(10),
            self.base.to_str_radix(10)
        )
    }
}
/*
keyset
*/
#[derive(Debug)]
pub struct KeySet {
    e: BigUint,
    d: BigUint,
    n: BigUint,
}
impl KeySet {
    // TODO: 키의 비트 크기를 구성 가능하게

    pub fn new() -> KeySet {
        let num_bits = 1024;
        let e = util::gen_prime(num_bits / 2);
        let d = util::gen_prime(num_bits / 2);
        let n = &e * &d;

        KeySet { e, d, n }
    }
}
pub fn create_file() {}

pub fn encrypt_file() {}

pub fn decrypt_file() {}
pub fn read_key_files() {}
