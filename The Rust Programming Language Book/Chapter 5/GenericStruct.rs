// Question: How can we use generic types with structs?

struct Pair<T> {
    first: T,
    second: T,
}

fn UseGenericStruct() {
    let int_pair = Pair { first: 1, second: 2 };
    let float_pair = Pair { first: 1.5, second: 2.5 };

    println!("Int Pair: {}, {}", int_pair.first, int_pair.second);
    println!("Float Pair: {}, {}", float_pair.first, float_pair.second);
}
