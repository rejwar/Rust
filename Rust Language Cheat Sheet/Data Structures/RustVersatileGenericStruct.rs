struct  S<T> {
    x : T,
}

fn main() {
    let s1 = S { x: 42};
    println!( " S1.x : {}",s1.x);

    let s2 = S { x : String::from("Hello World")};
    println!( " s2.x = {}", s2.x);

    let s3 = S{ x: 3.14};
    println!( " s3.x = {}", s3.x);
}
