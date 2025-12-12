fn main() {
    let t = (String::from("Move"), 100);
    let (s, i) = t;

    println!("String moved {}", s);
    println!("Int copied {}", i);
}
