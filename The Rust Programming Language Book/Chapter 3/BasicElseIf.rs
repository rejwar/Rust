use std::num::ParseIntError;

fn check_number(x:i32) {
    if x< 0 {
        println!("The number is Negative");
    } else if  x==0 {
        println!("The number is zero ");
    } else {
        println!("The number is positive :");
    }
}

fn main() {
    check_number(5);
    check_number(0);
    check_number(-5);
}
