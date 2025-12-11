#[allow(non_snake_case)]

fn main() {
    let data = String::from("Read Only");

    let r1 = &data;
    let r2 = &data;
    let r3 = &data;

    println!("{} {} {}", r1, r2, r3);
}
