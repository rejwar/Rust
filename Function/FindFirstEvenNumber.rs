fn find_first_even_number(numbers: &[i32]) -> Option<i32> {
    numbers.iter().find(|&&x| x % 2 == 0).copied()
}

fn main() {
    let arr = [1, 2, 3, 4, 5, 6, 7, 8];
    match find_first_even_number(&arr) {
        Some(num) => println!("First even number: {}", num),
        None => println!("No even numbers found"),
    }
}
