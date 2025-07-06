fn main() {
    let s1 = String::from("Hello , Rust");
    {
        let s2 = String::from("Inner Scope");
        println!("S2 : {}", s2);
    }
    println!("s1 :{}", s1);
}
