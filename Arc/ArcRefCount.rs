use std::sync::Arc;

fn main() {
    let SharedValue = Arc::new(100);
    
    let Clone1 = Arc::clone(&SharedValue);
    let Clone2 = Arc::clone(&SharedValue);

    println!("Reference Count: {}", Arc::strong_count(&SharedValue));
}
