fn main() {
    let mut x = 10;
    let mut add_one = || x += 1;
    add_one();
    println!("Updated {}", x);
}
