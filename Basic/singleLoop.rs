fn SumArray(arr: &[i32]) -> i32 {
    let mut sum = 0;

    for Value in arr {
        sum += Value;
    }

    sum
}

fn main() {
    let data = [1, 2, 3, 4, 5];
    println!("{}", SumArray(&data));
}
