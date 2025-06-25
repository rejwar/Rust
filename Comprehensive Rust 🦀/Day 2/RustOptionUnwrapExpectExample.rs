use std::collections::btree_map::Values;

fn main() {
    let Number: Option<i32> = Some(7);

    let Value = Number.unwrap();
    println!(" Value is {}", Value);

    let Value = Number.expect("No value found");
    println!("Value is {}", Value);
}
