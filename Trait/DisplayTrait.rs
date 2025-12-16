use std::fmt::Display;

fn print_anything<T: Display>(item: &T) {
    println!(" THe value is {}", item);
}
