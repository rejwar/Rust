fn main()
{
    let Title = String::from("Rust Book");

    let R1 = &Title;
    println!("Line  A  - {}", R1);

    println!("Line B -> {}", Title);

    let R2 = &Title;
    println!("Line -> c {}", Title);
}