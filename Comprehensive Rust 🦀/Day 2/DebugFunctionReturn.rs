fn Add( Num1: i32 , Num2:i32)->i32 {
    Num1 + Num2 
}

fn main() {
    let Result = dbg!(Add(10, 20));
    println!("Result {}", Result);
}
