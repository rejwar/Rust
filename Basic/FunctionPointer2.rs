fn Add(a: i32 , b: i32) -> i32
{
    a+b
}

fn Calculate (f: fn(i32 , i32) -> i32 , x: i32 , y:i32)
{
    println!("Result: {}", f(x,y));

}

fn main(){
    Calculate(Add,10,20);
}
