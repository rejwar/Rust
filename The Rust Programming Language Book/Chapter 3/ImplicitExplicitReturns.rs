fn ImplicitReturn(x:i32)-> i32{
    return x *2
}

fn ExplicitReturn(x:i32)-> i32 {
    return x*2;
}

fn main() {
    println!("Implict {} ", ImplicitReturn(10));
    println!("Explicit Return {}", ExplicitReturn(20));
}
