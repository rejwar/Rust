struct Container<T> {
    item: T,
}

impl <T> Container<T> {
    fn New(item: T) -> Self {
        Container { item }
    }

    fn Get(&self) -> &T {
        &self.item
    }
}

fn main() {
    let int_box = Container::New(42);
    let str_box = Container::New("Rust");

    println!("Int {}" , int_box.Get());
    println!("Int {}", str_box.Get());
    
}