fn main() {
    let Numbers: Vec<i32> = vec![1,2,3,4,5,6,7,78];
    let Doubled: Vec<i32> = Numbers.iter().map(|x| x*2).collect();

    println!("Doubled Values : {:?}", Doubled);
}
