use clap::{App, AppSettings, Arg, SubCommand};
use rsa_test::{create_file, decrypt_file, encrypt_file, read_key_files, Key, KeySet};

fn main() {
    println!("rsa알고리즘을 구현");
    let test = KeySet::new();
    println!("{:?}", test)
}
#[cfg(test)]
mod test {
    #[test]

    fn test() {}
}
