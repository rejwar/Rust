use std::sync::atomic::{AtomicUsize, Ordering};
fn main() {
    let atomic_val = AtomicUsize::new(10);

    println!(
        "Atomic current value is {}",
        atomic_val.load(Ordering::SeqCst)
    );
}
