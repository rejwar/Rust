use std::{clone, fmt::Display};

fn combine<T: Display +Clone> (a: T, b:T) -> T {
    println!("Combining is {} + {}", a, b);
a.clone()
}