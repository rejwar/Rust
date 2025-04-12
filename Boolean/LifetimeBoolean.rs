fn CompareStrings<'a>(Str1: &'a str, Str2: &'a str) -> bool {
    Str1 == Str2 // Returns true if strings are equal
}

fn main() {
    let FirstString: &str = "Rust";
    let SecondString: &str = "Rust";

    let IsEqual: bool = CompareStrings(FirstString, SecondString);
    println!("Are Strings Equal? {}", IsEqual);
}
