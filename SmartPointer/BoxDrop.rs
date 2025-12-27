struct SecretData;

impl Drop for SecretData {
    fn drop(&mut self) {
        println!("Secret data securely deleted from Heap");
    }
}

fn UseBoxDrop() {
    let _data = Box::new(SecretData);
    println!(" Secret data created ");
}

fn main() {
    UseBoxDrop();
}
