use clap::{App, AppSettings, Arg, SubCommand};
use rand::Rng;
use rsa_test::{create_file, decrypt_file, encrypt_file, read_key_files, Key, KeySet};
//Encryption
fn n_power(mut x: i32, mut e: i32, mut m: i32) -> i32 {
    let mut res: i32 = 1;

    while e > 0 {
        if (e % 2) == 1 {
            res = (res * x) % m;
            e = e - 1;
        } else {
            x = (x * x) % m;
            e = e / 2;
        }
    }
    res
}

fn is_bool(mut n: i32, mut k: i32) -> bool {
    if n % 2 == 0 && n != 2 {
        return false;
    } else if n <= 3 {
        return true;
    }
    while k > 0 {
        let mut rng = rand::thread_rng();
        let test: i32 = rng.gen();
        let a: i32 = test * (n - 3) + 2;

        if n_power(a, n - 1, n) != 1 {
            return false;
        }
    }

    return true;

    // while k > 0 {
    //     let mut rng = rand::thread_rng();
    //     let a: i32 = rng.gen() * (n - 3) + 2;
    //     if n_power(a, n - 1, n) != 1 {
    //         false
    //     } else {
    //         true
    //     }

    //     true
    // }
}
fn main() {
    let mut rng = rand::thread_rng();
    println!("{:?}", is_bool(1, 2));
    let x: u8 = rng.gen();
    println!("{:?}", x);
    // println!("{:?}", n_power(3, 2, 1))
    // println!("RSA Encryption");
    // let test = KeySet::new();
    // println!("{:?}", test);

    // static KEY_FILE_NAME: &'static str = "key";
    // static PUBLIC_KEY_SUFFIX: &'static str = "pub";
}

#[cfg(test)]
mod test {
    #[test]

    fn test() {}
}
