fn ModifyData(Data: &mut String) {
    Data.push_str(" - Updated!");
}

fn main() {
    let mut Info: String = String::from("Original Data");
    ModifyData(&mut Info);
    println!("{}", Info);
}
