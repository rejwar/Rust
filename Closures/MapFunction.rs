fn main() {
    let Numbers = vec![1, 2, 3, 4, 5];
    let Doubled: Vec<i32> = Numbers.iter().map(|x| x * 2).collect();

    println!("Doubled Numbers: {:?}", Doubled);
}
