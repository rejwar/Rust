use kyber::Kyber512;

fn main() {
    let (public_key, private_key) = Kyber512::keygen();
    let (ciphertext, shared_secret) = Kyber512::encapsulate(&public_key);
    
    let decrypted_secret = Kyber512::decapsulate(&ciphertext, &private_key);
    
    println!("Post-Quantum Secure Key Exchange Done!");
}
