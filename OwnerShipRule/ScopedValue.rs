{
    let scoped_owner = String::from("Scoped Value");
    println!("{}", scoped_owner); // Scoped variable
}
// After this block, scoped_owner is dropped.
