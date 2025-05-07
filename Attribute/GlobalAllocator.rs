use std::alloc::System;

#[global_allocator]
static GLOBAL: System = System;

fn main() {
    let Data = Box::new(42);
    println!("Allocated Data: {}", Data);
}
