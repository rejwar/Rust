// Question: How do you declare a mutable variable without assigning it immediately?

fn main() {
    let mut x: i32; // declare as mutable

    x = 10; // initialize

    println!("x = {}", x); // ✅ works
}
