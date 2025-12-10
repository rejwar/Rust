fn main() {
    let mut basket = vec![String::from("Apple")];

    let fruit = basket.pop().unwrap();

    println!(" I got {}", fruit);
    println!(" Basket len {}", basket.len());
}
