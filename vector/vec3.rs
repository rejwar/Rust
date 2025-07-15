fn main() {
    let mut Numbers = vec![12,13,14,15,16];
    Numbers.sort();
    println!("Sorted : {:?}", Numbers);

    Numbers.reverse();
    println!("Rversed {:?}", Numbers);
}