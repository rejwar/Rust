use ntru::NtruEncrypt;

fn main() {
    let (public_key, private_key) = NtruEncrypt::keygen();
    let message = b"Quantum-Safe Encryption!";
    
    let encrypted_message = NtruEncrypt::encrypt(message, &public_key);
    let decrypted_message = NtruEncrypt::decrypt(&encrypted_message, &private_key);
    
    println!("Decrypted Message: {:?}", String::from_utf8(decrypted_message));
}
