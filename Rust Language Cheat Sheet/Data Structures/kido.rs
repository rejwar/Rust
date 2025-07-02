// InclusiveRange.rs: How to use inclusive range (..=b) in Rust?

fn main() {
    let b = 5;

    // (1) Iterating over a range
    println!("Iterating over range ..=b:");
    for i in ..=b {
        println!("{}", i);
    }

    // (2) Using with arrays
    let arr = [10, 20, 30, 40, 50];
    let slice = &arr[..=b];
    println!("Array slice: {:?}", slice);

    // (3) Using with pattern matching
    let x = 3;
    match x {
        ..=5 => println!("x is less than or equal to 5"),
        _ => println!("x is greater than 5"),
    }
}
