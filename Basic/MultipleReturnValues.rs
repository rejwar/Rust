use std::iter::Product;

fn Calculate (a: i32, b: i32)-> (i32, i32)
{
    (a+b, a*b)
}

fn main(){
    let (sum , product) = Calculate(5, 6);
    println!("Sum : {}, Product: {}", sum, product);
}
