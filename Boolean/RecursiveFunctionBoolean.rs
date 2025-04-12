fn HasEvenNumber(Numbers: &[i32]) -> bool {
    if Numbers.is_empty() {
        false
    } else if Numbers[0] % 2 == 0 {
        true
    } else {
        HasEvenNumber(&Numbers[1..]) // Recursively check remaining numbers
    }
}

fn main() {
    let Data: Vec<i32> = vec![1, 3, 5, 7, 10];
    let ContainsEven: bool = HasEvenNumber(&Data);
    println!("Contains Even Number: {}", ContainsEven);
}
