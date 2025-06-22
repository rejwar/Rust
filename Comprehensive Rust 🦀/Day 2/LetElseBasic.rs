use std::num::ParseIntError;

fn main() {
    let opt: Option<i32> = Some(32);

    let Some(value ) = opt else {
        println!("No value found , exiting ");
        return;
    };
    println!("Got value {}", value);

}
