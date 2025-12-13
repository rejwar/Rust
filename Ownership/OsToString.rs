// FileName: OsToString.rs
use std::ffi::OsString;
fn main() {
    let os = OsString::from("File");
    let s = os.into_string().unwrap();
    println!("{}", s);
}
