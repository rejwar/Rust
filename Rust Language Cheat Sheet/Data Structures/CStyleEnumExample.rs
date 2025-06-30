#[repr(u8)]

enum E {
    A =1,
    B = 2,
    C = 4,
}

fn main() {
    let x = E::B as u8;
    println!("The discount is {}",x);
}
