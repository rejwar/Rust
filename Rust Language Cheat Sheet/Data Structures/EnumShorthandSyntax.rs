// Question: Can I use shorthand `E::C { x }` when local variable `x` exists?

enum Data {
    Value { num: i32 },
}

fn main() {
    let num = 100;

    // Uses shorthand: num field gets the value from `num` variable
    let d = Data::Value { num };

    match d {
        Data::Value { num: val } => {
            println!("Value is {}", val);
        }
    }
}
