fn main() {
    let mut Data: i32 = 100;

    let R1 = &Data;
    let R2: &i32 = &Data;
    println!("{} {}", R1, R2);
}