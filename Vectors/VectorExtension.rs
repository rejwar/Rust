fn main() {
    let mut Vec1: Vec<i32> = vec![1,2,3];
    let Vec2: Vec<i32> = vec![4,5,6];

    Vec1.extend(Vec2);
    println!("Combined Vector {:?}", Vec1);
}
