use std::result;

fn apply_operation<F> (operation : F , x: i32 , y: i32 ) -> i32 
    where 

    F : Fn(i32 , i32 ) -> i32 ,
{
    operation(x,y)
} 

fn main() {
    let add = |x : i32 , y: i32 | -> i32 {x +y};

    let result = apply_operation(add, 5, 10);
    println!("Result {}", result);
}