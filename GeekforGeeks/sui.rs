fn main() {
    let mut Age = 20;
    Bump(&mut Age);
    println!("Age -> {}", Age);
}

fn Bump(X: &mut i32 ){
    *X += 4;
}