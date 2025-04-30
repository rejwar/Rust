fn ModifyReference(Number: &mut i32 ) {
    *Number += 10;
}

fn main() {
    let mut Value :i32 = 50;
    ModifyReference(&mut Value);
    println!("Updated Value {}",Value);
}
