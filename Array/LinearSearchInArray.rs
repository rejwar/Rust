fn LinearSearchInArray( arr: [i32;5] , target:i32 ) -> bool {
    for &item in arr.iter() {
        if item == target {
            return true;
        }
    }
    false
}

fn main() {
    let data = [3,4,5,6,7];
    let found = LinearSearchInArray(data, 5);
    println!("Is 5 found {}" , found);
}
