struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping Custom Smart Pointer `{}`", self.data);
    }
}

fn main() {
    let c = CustomSmartPointer {
        data: String::from("MyStuff"),
    };
}
