use concrete::prelude::*;

pub fn homomorphic_addition(value: i32, addend: i32) -> i32 {
    let sk = SecretKey::new();
    let encrypted_value = sk.encrypt(value);
    let result = encrypted_value.add(addend);
    result.decrypt(&sk)
}

fn main() {
    println!("Zero-Config Homomorphic Computation: {}", homomorphic_addition(42, 8));
}
