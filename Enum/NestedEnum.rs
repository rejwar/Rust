enum OuterEnum {
    Variant1,
    InnerEnum(InnerEnum),
}

enum InnerEnum {
    Inner1,
    Inner2,
}

fn main() {
    let value = OuterEnum::InnerEnum(InnerEnum::Inner1);
    match value {
        OuterEnum::Variant1 => println!("Outer variant 1"),
        OuterEnum::InnerEnum(inner) => match inner {
            InnerEnum::Inner1 => println!("Inner enum 1"),
            InnerEnum::Inner2 => println!("Inner enum 2"),
        },
    }
}
