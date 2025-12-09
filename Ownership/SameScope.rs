struct MyData {
    name: String,
}

impl Drop for MyData {
    fn drop(&mut self) {
        println!("Dropping {} ", self.name);
    }
}

fn main() {
    let x = MyData {
        name: String::from("A"),
    };
    let y = MyData {
        name: String::from("B"),
    };

    println!("ENd of main");
    c
}
