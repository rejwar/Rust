use dilithium::{Dilithium512, Signature};

fn main() {
    let (public_key, secret_key) = Dilithium512::keygen();
    let message = b"Secure Quantum-Safe Signature!";
    let signature = Dilithium512::sign(message, &secret_key);

    assert!(Dilithium512::verify(message, &signature, &public_key));
    println!("Post-Quantum Signature Verified Successfully!");
}
