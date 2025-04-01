use std::iter::Sum;

fn SumTwoNumbers(a: i32 , b:i32 ) -> i32
{
    a+ b     
}

fn main() {
    let result = SumTwoNumbers(5, 10);
    println!("Sum : {} ", result); 
}
