enum DefaultVariant {
    Yes,
    No,
}

fn main() {
    let variant = DefaultVariant::No;
    match variant {
        DefaultVariant::Yes => println!("Yes"),
        DefaultVariant::No => println!("No"),
    }
}
