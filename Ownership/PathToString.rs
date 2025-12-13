// FileName: PathToString.rs
use std::path::PathBuf;
fn main() {
    let p = PathBuf::from("/tmp");
    let s = p.into_os_string().into_string().unwrap();
    println!("{}", s);
}
