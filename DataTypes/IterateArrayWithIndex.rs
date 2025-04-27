fn main() {
    let arr = [10,20,30,40,50];

    for (index ,value) in arr.iter().enumerate() {
        println!("Index: {} , Value {}" , index, value);
    }
}
