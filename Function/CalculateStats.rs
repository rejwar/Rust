fn Stats(x:i32 , y:i32) -> (i32,i32) {
    (x+y , x*y)
}

fn main() {
    let (sum, product) = Stats(4, 5);
    println!("Sum : {} , Product: {}", sum, product);
}
