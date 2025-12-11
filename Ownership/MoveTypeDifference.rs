fn main() {
    let x = 10;
    let y = x;
    println!("x is alive {}", x);

    let s1 = String::from("Hi");
    let s2 = s1;

    println!("s1 is dead {}", s2);
}
