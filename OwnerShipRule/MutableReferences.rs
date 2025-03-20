let mut value = String::from("Mutable Reference");
let reference = &mut value;
reference.push_str(" - Updated");
println!("{}", reference);
