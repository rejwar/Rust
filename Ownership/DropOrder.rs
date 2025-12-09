struct MyData {
    name: String,
}

impl Drop for MyData {
    fn drop(&mut self) {
        println!("Dropping {}", self.name);
    }
}

fn main() {
    let a = MyData {
        name: String::from("A"),
    };
    let b = MyData {
        name: String::from("B"),
    };

    println!("THe main function Ending");
}
