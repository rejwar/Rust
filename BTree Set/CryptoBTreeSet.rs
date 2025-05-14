use std::collections::BTreeSet;
use sha3::{Digest, Sha3_256};

fn hash_data(data: &str) -> String {
    format!("{:x}", Sha3_256::digest(data.as_bytes()))
}

fn main() {
    let mut SecureSet = BTreeSet::new();
    SecureSet.insert(hash_data("Alice"));
    SecureSet.insert(hash_data("Bob"));

    println!("Hashed User Data: {:?}", SecureSet);
}
