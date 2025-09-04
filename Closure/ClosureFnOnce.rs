fn ApplyOnce<F: FnOnce()> (f: F) {
    f();
}

fn main() {
    let Name = String::from("Md .");
    let Consume = move || println!("Goodbye  {}", Name);

    ApplyOnce(Consume);
}