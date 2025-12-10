struct MyData {
    val: i32,
}

impl Drop for MyData {
    fn drop(&mut self) {
        println!("Claening up auromaetcally ");
    }
}

fn main() {
    let _data = MyData { val: 10 };
}
