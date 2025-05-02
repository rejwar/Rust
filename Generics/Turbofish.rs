fn GetNumber<T: std::str::FromStr>(Input: &str) -> Option <T> {
    Input.parse::<T>(). ok()
}

fn main() {
    let Number: Option<i32> = GetNumber::<i32> ("42");
    println!("Parsed Number : {:?}", Number);
}
