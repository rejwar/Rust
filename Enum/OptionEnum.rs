fn FindIndex (value: i32 , arr: &[i32]) -> Option<usize>
{
    for (i , &v) in arr.iter(). enumerate(){
        if v == value{
            return Some(i);
        }
    }
    None
}

fn main() {
    let nums = [3,5,7];
    match FindIndex(5, &nums){
        Some(index) => println!("Found at {}", index),
        None => println!("Not found"),
    }
}
