fn ReturnTuple() -> (i32, ) {
    (42,)
}

fn main() {
    let value = ReturnTuple();
    println!("Returned Tuple {:?}", value);
}
