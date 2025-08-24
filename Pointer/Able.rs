fn main() {
    let mut Number = 50;

    let RefOne = &Number;
    let RefTwo: &i32 = &Number;

    println!("Immutable {} {}", RefOne , RefTwo);

    let RefMut = &mut Number;
    *RefMut +=25;
    println!("Updated Numebr = {}", Number);
}