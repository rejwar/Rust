fn main() {
    let numbers = vec![1,2,3,4,5,6,7];

    let sqaures: Vec<i32> = numbers.iter().map(|x| x *x).collect();

    println!(" Squares is {:?}", sqaures);
}
