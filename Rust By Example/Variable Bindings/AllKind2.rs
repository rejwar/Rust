fn main() {
    let mut x = 5;
    println!("Before mutation , x {}", x);

    x += 10;
    println!("After mutation , x :{}", x);

    let x = "Hello";
    println!("After shadowing ,x: {}",x);
}
