// FileName: OptionMatch.rs
fn main() {
    let opt = Some(String::from("Val"));
    match opt {
        Some(s) => println!("Owned: {}", s), // s moved here
        None => (),
    }
    // println!("{:?}", opt); // Error: partially moved
}
