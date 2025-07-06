fn main() {
    let x = 5;

    {
        let x = x+1;
        println!("Inner scope x : {}", x);
    }
    println!("Outer scope is {}", x);
}
