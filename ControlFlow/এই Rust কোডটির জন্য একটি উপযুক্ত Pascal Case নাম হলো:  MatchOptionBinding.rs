use std::collections::btree_map::Values;

fn BindingMatchExample(opt : Option<i32>) {
    match opt {
        Some(Value) => println!("GOt Value :{}", Value),
        None => println!("Got Nothing"),
    }
}


fn main() {
    BindingMatchExample(Some(42);
    BindingMatchExample(None);
}
