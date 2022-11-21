mod util;

use num_bigint::BigUint;
use std::cmp::Ordering;
use std::error::Error;
use std::fs::{self, File};
use std::io::prelude::*;
use std::str;
#[derive(Debug)]
pub struct KeySet {
    e: BigUint,
    d: BigUint,
    n: BigUint,
}
impl KeySet {}
pub fn create_file() {}

pub fn encrypt_file() {}

pub fn decrypt_file() {}
pub fn read_key_files() {}
