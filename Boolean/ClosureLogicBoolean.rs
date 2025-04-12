fn main() {
    let ValidateNumber = |Number: i32| -> bool {
        Number > 0 && Number % 2 == 0
    };

    let TestNumber: i32 = 8;
    let IsValid: bool = ValidateNumber(TestNumber);

    println!("Is {} A Positive Even Number? {}", TestNumber, IsValid);
}
