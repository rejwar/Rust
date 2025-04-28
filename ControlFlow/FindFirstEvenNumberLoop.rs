fn FindFirstEvenNumber(numbers: &[i32]) ->i32 {
    let Result = loop {
        for &num in numbers {
            if num % 2 == 0{
                break num;
            }
        }

        break-1;
    };
}

fn main() {
    let arr = [1,2,3,4,5,6,7,8];
    println!("First even number {}",FindFirstEvenNumber(&arr));
}
