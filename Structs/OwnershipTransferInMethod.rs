struct Data {
    value: String,
}

impl Data {
    fn into_inner(self) -> String {
        self.value
    }
}

fn main() {
    let data = Data {
        value: String::from("Ownership Transferred"),
    };
    let inner_value = data.into_inner();
    println!("{}", inner_value);
}
