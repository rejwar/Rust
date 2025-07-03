fn main() {
    let mut value = 10;

    {
        let r: &mut i32 = &mut value;
        *r += 5;

    }
    println!("Updated value is {}", value);
}
