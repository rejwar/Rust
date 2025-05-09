use kyber::Kyber512;

fn main() {
    let (public_key, secret_key) = Kyber512::keygen();
    let (ciphertext, shared_secret) = Kyber512::encapsulate(&public_key);

    println!("Post-Quantum Secure Key Exchange Completed!");
}
