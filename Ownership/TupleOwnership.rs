fn main() {
    let mut my_tuple = (10, 20);

    let first = &mut my_tuple.0;
    let second = &mut my_tuple.1;

    *first += 5;

    println!(" Tuple  {:?} ", my_tuple);
}
