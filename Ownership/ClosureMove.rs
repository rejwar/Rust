fn main() {
    let s = String::from("Owned");
    let take = move || println!(" Taken {}", s);
    take();
}
