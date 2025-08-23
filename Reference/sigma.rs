fn main() {
    let mut age = 30;
    addOne(&mut age);
    println!("Updated age: {}", age);
}

fn addOne(a: &mut i32 ) {
    *a += 1;
}