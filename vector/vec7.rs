use std::vec;

fn main() {
    let mut Data = Vec::with_capacity(10);
    println!("Initial capacity {}", Data.capacity());


    Data.push(1);
    Data.push(2);
    println!("After push {:?}", Data);
    println!("Updated capacity {}", Data.capacity());
}