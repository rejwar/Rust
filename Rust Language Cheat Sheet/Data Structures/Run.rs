// Question: How can I declare a variable first, then assign a value?

fn main() {
    let x: i32;
    
    if true {
        x = 42;
    }

    println!("x = {}", x); // ✅ Works, because x was assigned before use
}
