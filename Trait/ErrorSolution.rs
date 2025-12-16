struct DataRef<'a> {
    data: &'a str,
}

impl<'a> DataRef<'a> {
    fn get_data(&self) -> &str {
        self.data
    }
}

fn main() {
    let text = String::from("Rust Lifetime");
    let ref_struct = DataRef { data: &text };
    println!(" {}", ref_struct.get_data());
}
