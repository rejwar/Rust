use rayon::prelude::*;
use sha2::{Sha256, Digest};

fn main() {
    let data = vec!["Alice", "Bob", "Charlie"];
    
    let hashes: Vec<String> = data.par_iter()
        .map(|input| format!("{:x}", Sha256::digest(input.as_bytes())))
        .collect();

    println!("Parallel Cryptographic Hashing Done: {:?}", hashes);
}
