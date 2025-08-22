fn GiveRef() -> &String {
    let Temp: String = String::from("Dangling Ref");
    &Temp
}
fn main() {
    let Ref &String = GiveRef();
    println!("{}", Ref);
}