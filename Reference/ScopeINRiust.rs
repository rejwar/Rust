fn main() {
    let mut Title = String::from("Rust Bopok");

    {
        let R1:&String = &Title;
        let R2:&String = &Title;

        println!("Readers is {} {}",R1,R2  );
    }

    Title.push_str("!");
    println!("Owner is {}", Title);
}