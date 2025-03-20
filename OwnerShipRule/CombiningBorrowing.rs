let mut owner = String::from("Conflict Borrow");
let immutable_borrow = &owner; // Immutable borrow
// let mutable_borrow = &mut owner; // Error: cannot borrow as mutable while immutable borrow exists.
println!("{}", immutable_borrow);
