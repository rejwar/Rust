use std::vec;

fn main() {
    let mut vec_integer: Vec<i32> = vec![1,2,3,45,6];
    vec_integer.push(50);
    vec_integer.push("Hello");
    //error mismatched type;
    println!( " {:?}", vec_integer);
}
