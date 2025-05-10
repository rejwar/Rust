use sha3::{Digest, Sha3_256};

pub fn quick_hash(data: &str) -> String {
    format!("{:x}", Sha3_256::digest(data.as_bytes()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hashing() {
        assert_eq!(quick_hash("RustCrypto"), quick_hash("RustCrypto"));
    }
}
