use std::fmt::Error;

fn GetValue(Value: Option<i32>) -> i32 {
    match Value {
        Some(Number ) => Number,
        None => panic!("No valid value Found"),
    }
}

fn main() {
    let Num = GetValue(Some(42));
    println!("Number {}" , Num);

    let ErrorNum = GetValue(None);
}
