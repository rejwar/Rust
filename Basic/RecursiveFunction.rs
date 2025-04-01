fn Factorial(n: u32) ->u32
{
    if n == 0{
        1
    } else{
        n* Factorial(n-1)
    }
}

fn main(){
    let result = Factorial(5);
    println!("Factorial: {}", result);
}
