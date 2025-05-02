use std::{collections::btree_map::Values, vec::Splice};

fn FindNumber(Number :i32 ) -> Option<i32> {
    if Number > 0 {
        Some(Number)
    } else {
        None
    }
}

fn main() {
    match FindNumber(6){
        Some(Values) => println!("Found  :{}" , Values),
        None => println!("No valid Number "),
    }
}
