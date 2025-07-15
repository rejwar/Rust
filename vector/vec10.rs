use std::fs::File;

fn main() {
    let mut Number = vec![10,15,20,25,30];

    Number.retain(|&x| x % 20 != 0);
    println!("After retain : {:?}", Number);

    let Filtered: Vec<_> = Number.into_iter().filter(|x| x > &15).collect();
    println!("Filtered {:?}", Filtered);
}