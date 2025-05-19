fn FindMaxInArray(arr: [i32;6]) -> i32 {
    let mut max = arr[0];

    for &num in arr.iter() {
        if num > max {
            max = num;
        }
    }

    max 
}

fn main() {
    let data = [12,13,1,1,5,9];
    let max_value = FindMaxInArray(data);
    println!("Maximum value : {}", max_value);
}
