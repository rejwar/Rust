let owner = String::from("Immutable Borrow");
let borrowed = &owner; // Borrow immutably
println!("Owner: {}, Borrowed: {}", owner, borrowed);
