fn main() {
    let x =10;
    {
        let x = x*2;
        println!("Inner scope x :{}", x);
    }

    println!("Outer scope is {}", x);
}
