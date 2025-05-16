fn main()  {
    let s = String::from("Hello");
    println!("s = {}", s);
    let s2 = s.clone();
    println!("s2 = {}", s2);
    // println!("s = {}", s); // This line would cause a compile-time error because s has been moved
}
