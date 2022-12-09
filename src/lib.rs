mod util;
//큰 숫자
use num_bigint::BigUint;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::str;
/*
RSA
1.p는 q와 거긔 같은 크기이다.
2.p-1과 q-1은 큰 소인수를 같는다
3.p-1과 q-1의 최대 공약수는 작은 수이다.
4.d는 n과 거의 같은 크기이다.
*/
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
(e*d)mod phi(n)=1
e: 1<e <phi(n)로써 1과 phi(n)사이에 있고 phi(n)과 서로소인 e는 공개키에 이용
d:(e*d)/phi(n)나우었을떄 나머지가 1인 값,개인키에 사용될 값
n:임의의 두소수 p와 q를 정하고 n=p*q

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
        let p = util::gen_prime(num_bits / 2);
        let q = util::gen_prime(num_bits / 2);
        let n = &p * &q;

        let maxpq = match p.cmp(&q) {
            Ordering::Less => &q,
            _ => &p,
        };
        let phi_n = (&p - 1u32) * (&q - 1u32);
        let d = util::gen_prime_abrove(100, &maxpq);
        let e = util::mult_inverse(&phi_n, &d);
        KeySet { e, d, n }
    }
    fn from_key() {}
    /*
    임호화


    */
    fn encrypt(&self, msg: &BigUint) -> BigUint {
        //Returns (self ^ exponent) % modulus.
        msg.modpow(&self.e, &self.n)
    }
    /*
    복호화



    */

    fn decrypt(&self, cipher: &BigUint) -> BigUint {
        cipher.modpow(&self.d, &self.n)
    }

    pub fn get_private_key(&self) -> Key {
        Key {
            exponent: self.d.clone(),
            base: self.n.clone(),
        }
    }
    pub fn get_pubilc_key() {}
}

pub fn encrypt_file() {}

pub fn decrypt_file() {}
pub fn create_file() {}
pub fn read_key_files() {}
fn read_key_file() {}
