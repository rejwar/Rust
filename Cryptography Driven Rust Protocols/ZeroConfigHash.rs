use sha3::{Digest, Sha3_256};

pub fn hash_data(data: &str) -> String {
    format!("{:x}", Sha3_256::digest(data.as_bytes()))
}

fn main() {
    println!("SHA3-256 Hash: {}", hash_data("Zero-Config Encryption!"));
}
