struct Container<T> {
    value: T,
}

fn GenericStruct() {
    let IntContainer = Container { value: 10 };
    let StrContainer = Container { value: "Rust"};

    println!("{} {}" , IntContainer.value,StrContainer.value);
}
