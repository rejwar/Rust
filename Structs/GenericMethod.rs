struct Container<T> {
    item: T,
}

impl<T> Container<T> {
    fn get(&self) -> &T {
        &self.item
    }
}

fn main() {
    let container = Container { item: 42 };
    println!("Item: {}", container.get());
}
