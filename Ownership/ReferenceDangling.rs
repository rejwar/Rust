fn main() {
    let a: &i32;
    {
        let b:i32 = 3;
        a = &b;
        println!("{}", b);
    }

    println!("{}", a);
    println!("{}", *a);
}
