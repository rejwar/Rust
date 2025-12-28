use std::sync::Arc;

fn TrackArcCount() {
    let data = Arc::new(100);

    println!(" Initital count {}", Arc::strong_count(&data));

    let a1 = Arc::clone(&data);
    println!("After clone 1 = {}", Arc::strong_count(&data));

    {
        let a2 = Arc::clone(&data);
        println!("Inside block clone {}", Arc::strong_count(&data));
    }

    println!("After block end {}", Arc::strong_count(&data));
}

fn main() {
    TrackArcCount();
}
