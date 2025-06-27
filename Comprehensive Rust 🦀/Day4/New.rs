fn main() {
    let result: i32 = (1..)
    .filter(|x| x % 3 == 0 )
    .take(5)
    .sum();
}
