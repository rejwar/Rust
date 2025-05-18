fn main() {
    let mut data = String::from("Rust");

    let r1 = &data;
    let r2 = &data;
    let r3= &mut data;
    println!("{} ,{} , {}" , r1,r2,r3);
}
