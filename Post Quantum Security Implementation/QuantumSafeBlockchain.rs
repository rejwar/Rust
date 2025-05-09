use pqcrypto::sign::dilithium2;

fn main() {
    let (public_key, private_key) = dilithium2::keypair();
    let transaction = b"Blockchain Transaction Data";

    let signature = dilithium2::sign(transaction, &private_key);
    assert!(dilithium2::verify(transaction, &signature, &public_key));
    
    println!("Quantum-Safe Blockchain Transaction Verified!");
}
