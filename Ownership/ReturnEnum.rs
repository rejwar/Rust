// FileName: ReturnEnum.rs
enum Res {
    Success(String),
}
fn do_work() -> Res {
    Res::Success(String::from("Done"))
}
fn main() {
    let r = do_work();
}
