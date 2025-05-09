use concrete::prelude::*;

fn main() {
    let secret_key = SecretKey::new();
    let encrypted_value = secret_key.encrypt(42);
    let processed_value = encrypted_value.add(10); // ✅ Computation without decryption!

    println!("Homomorphic Computation Done: {:?}", processed_value.decrypt(&secret_key));
}
