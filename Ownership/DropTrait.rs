// FileName: DropTrait.rs
struct Custom;
impl Drop for Custom {
    fn drop(&mut self) {
        println!("Dropping!");
    }
}
fn main() {
    let _c = Custom;
} // Prints "Dropping!" here
