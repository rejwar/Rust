fn IsEqual<T: PartialEq>(ValueOne: T, ValueTwo: T) -> bool {
    ValueOne == ValueTwo
}

fn main() {
    let IsSame: bool = IsEqual(10, 10);
    println!("Are The Values Equal? {}", IsSame);
}
