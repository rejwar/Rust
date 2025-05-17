fn main() {
    let arr: [i32;4] = [10,20,30,40];
    println!("Array is {:?}",arr);
    println!("array is :{}",arr.len());

    for index in 0..4 {
        println!("Index is : {} & value is : {}", index,arr[index]);
    }
}
