struct Pair<T, U> {
    first: T,
    second: U,
}

fn GenericStructedMultiple () {
    let pair = Pair {
        first: "Height",
        secomd: 100,
    };

    println!("{} {}" , pair.first , pair.second);
}
