fn main() {
    let Slice: &str  = "Rustacean";
    let Owned: String = Slice.to_string();

    let BackToSlice: &str= &Owned;
    println!("Owned :{} , Slice {}", Owned,BackToSlice);
}
