use std::collections::btree_map::Values;

fn FindEven(Number: i32 ) -> Option<i32> {
if Number % 2 == 0 {
    Some((Number))
} else {
    None
}
}

fn main() {
    match FindEven(10) {
        Some(Values) => println!("Even Number : {}", Values),
        None=> println!("Not an Even Number "),
    }
}
