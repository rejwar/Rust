use sha3::{Digest, Sha3_256};

fn ComputeHash(data: &str) -> String {
    let mut hasher = Sha3_256::new();
    hasher.update(data);
    format!("{:x}", hasher.finalize())
}

fn main() {
    let hash = ComputeHash("RustCryptography");
    println!("SHA3-256 Hash: {}", hash);
}
