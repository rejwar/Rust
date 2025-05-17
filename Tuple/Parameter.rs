fn main() {
    let b:(i32 , bool,f64) = (110,true, 10.9);
    print(b);
}

fn print(x:(i32,bool,f64)) {
    println!("Inside print number ");
    println!("{:?}", x);
}
