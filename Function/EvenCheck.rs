fn main() {
    println!("{}" , is_even(4));
}

pub fn is_even(n: i32) -> bool {
    n % 2 == 0
}
