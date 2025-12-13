// FileName: ResultMatch.rs
fn main() {
    let res: Result<String, _> = Ok(String::from("Ok"));
    if let Ok(s) = res {
        println!("Moved: {}", s);
    }
}
