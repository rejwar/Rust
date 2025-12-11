use std::mem::take;

fn main() {
    let s = String::from("Go");
    take(s);
}

fn take(_input: String) {}
