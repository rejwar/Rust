#[derive(Debug)]

struct Boxed<T>{
    value: T,
}

fn main() {
    let a = Boxed {value : 42};
    let b = Boxed {value : "Hello"};
    println!("{:?} \n {:?}" , a,b );
}