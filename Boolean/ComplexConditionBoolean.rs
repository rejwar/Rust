fn main() {
    let Age: i32 = 21;
    let IsStudent: bool = true;

    let DiscountEligible: bool = Age < 25 && IsStudent; // Combining conditions
    println!("Eligible For Discount: {}", DiscountEligible);
}
