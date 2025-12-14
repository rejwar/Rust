struct Box<T> {
    Value: T,
}

fn main() {
    let int_box = Box { Value: 10 };
    let str_box = Box {
        Value: String::from("Hi"),
    };
}
