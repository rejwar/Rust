fn main() {
    let mut x =10;
    {
        x +=5;
        println!("Inner scope is {}",x);
    }
    println!("Outer scope is {}", x);
}
