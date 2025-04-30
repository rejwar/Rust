fn main() {
    let Ref;
    {
        let TempData: String = String::from("Rust");
        Ref = &TempData;
    }

    println!("{}",Ref);
}
