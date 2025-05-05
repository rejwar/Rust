fn main() {
    let Numbers = vec![1, 2, 3, 4, 5];
    let EvenNumbers: Vec<i32> = Numbers.iter().filter(|&&x| x % 2 == 0).collect();

    println!("Even Numbers: {:?}", EvenNumbers);
}
