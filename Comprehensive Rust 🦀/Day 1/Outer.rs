fn main() {
    let x = 5;

    {
        let x = x*2;
        println!("Inner x: {}", x); // This will print 10
    }

    println!("Outer x: {}", x); // This will print 5
}
