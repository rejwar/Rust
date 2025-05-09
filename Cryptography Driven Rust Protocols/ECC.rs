use curve25519_dalek::scalar::Scalar;
use curve25519_dalek::ristretto::RistrettoPoint;

fn main() {
    let scalar = Scalar::from(10u64);
    let base_point = RistrettoPoint::default();

    let public_key = base_point * scalar;
    println!("Generated ECC Public Key: {:?}", public_key);
}
