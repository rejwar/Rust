fn main() {
    let Numbers: Vec<i32> = vec![1, 2, 3, 4, 5];
    let EvenNumbers: Vec<i32> = Numbers.into_iter().filter(|&Number| Number % 2 == 0).collect();

    println!("Even Numbers: {:?}", EvenNumbers);
}
