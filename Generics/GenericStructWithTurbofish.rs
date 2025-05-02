struct Container<T> {
    Value: T,
}

fn main() {
    let BoxedValue = Container::<i32> { Value: 10 };
    println!("Stored Value: {}", BoxedValue.Value);
}
