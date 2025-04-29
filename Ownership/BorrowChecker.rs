fn main() {
    let mut Number: i32 = 10;
    let FirstBorrow: &i32 = &Number;
    // let SecondBorrow: &mut i32 = &mut Number; // ❌ Error: Borrow checker prevents mutation

    println!("First Borrow: {}", FirstBorrow);
}
