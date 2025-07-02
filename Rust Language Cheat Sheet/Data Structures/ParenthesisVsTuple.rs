fn main() {
    let x = 100;

    let not_a_tuple = (x);
    let tuple = (x,);

    println!("Not a tuple is {}", not_a_tuple);
    println!("Tuple is {:?}",tuple);
}
