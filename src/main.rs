use clap::{App, AppSettings, Arg, SubCommand};
use rsa_test::{create_file, decrypt_file, encrypt_file, read_key_files, Key, KeySet};
//Encryption
fn main() {
    println!("RSA Encryption");
    let test = KeySet::new();
    println!("{:?}", test);

    static KEY_FILE_NAME: &'static str = "key";
    static PUBLIC_KEY_SUFFIX: &'static str = "pub";
}
#[cfg(test)]
mod test {
    #[test]

    fn test() {}
}
