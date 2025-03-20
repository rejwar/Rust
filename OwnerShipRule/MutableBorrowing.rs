let mut owner = String::from("Mutable Borrow");
let borrowed = &mut owner; // Borrow mutably
borrowed.push_str(" - Modified");
println!("{}", borrowed);
