fn main() {
    let Result = AddAndSqaure(3,4);
    println!("Result  = {}", Result);
}

fn AddAndSqaure (A: i32 , B: i32 ) -> i32 {
    let Sum = A +B;
    let Square = Sum * Sum;
    return Square;
}