fn ModifyData(Data: &mut String) {
    Data.push_str(" Modified Inside Function!");
}

fn main() {
    let MutInfo: String = String::from("Original Data");
    ModifyData(&mut MutInfo);
    println!("{}", MutInfo);
}
