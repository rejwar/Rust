use kyber::Kyber512;

pub fn encrypt_message(msg: &[u8]) -> (Vec<u8>, Vec<u8>) {
    let (pk, sk) = Kyber512::keygen();
    let (ciphertext, shared_secret) = Kyber512::encapsulate(&pk);
    (ciphertext, shared_secret)
}
