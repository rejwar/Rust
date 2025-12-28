struct Secret {
    key: Vec<u8>,
}

impl Drop for Secret {
    fn drop(&mut self) {
        for b in &mut self.key {
            *b = 0;
        }

        println!(" Secret wiped before free");
    }
}

fn UseBoxSecureDrop() {
    let s = Box::new(Secret {
        key: vec![12, 3, 4, 5, 6],
    });
    println!(" Secret is memory ");
}

fn main() {
    UseBoxSecureDrop();
}
