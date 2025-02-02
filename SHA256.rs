use crypto::digest::Digest;
use crypto::sha2::Sha256;

fn main() {
    let mut hasher = Sha256::new();
    hasher.input_str("hello, world");
    let result = hasher.result_str();
    println!("SHA-256 hash: {}", result);
}
