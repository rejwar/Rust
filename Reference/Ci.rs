use serde_json::Value;

fn main() {
    let mut value = 4;
    Inc(&mut Value);
    Double(&mut Value);
    println!("value -> {}", Value);
}

fn Inc(X: &mut i32 ) {*X += 1;}
fn Double (X: &mut i32 ) {*X *=2;}