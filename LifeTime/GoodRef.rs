struct Data { Value: String}
fn GetRef<'a>(Input: &'a Data) -> &'a str {&Input.Value}

fn main() {
    let D = Data { Value: String::from("RUst")};
    let R = GetRef(&D);
    println!("{}", R);
}