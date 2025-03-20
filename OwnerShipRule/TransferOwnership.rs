let owner = String::from("Owned String");
let new_owner = owner; // Ownership moves to new_owner
// println!("{}", owner); // This would throw an error, as owner is no longer valid.
println!("{}", new_owner);
