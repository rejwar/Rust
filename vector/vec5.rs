fn main() {
    let mut Numbers = vec![10,15,20,25,30];
    Numbers.retain(|&x| x & 20 != 0);
    println!("After retain : {:?}", Numbers);

    let Filtered: Vec<_> =Numbers.into_iter().filter(|x| x > &15).collect();
    println!("Filtered {:?}", Filtered);
}