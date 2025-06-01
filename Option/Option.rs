fn GetNumber(flag: bool) -> Option<i32> {
    if flag {
        Some(100) 

        
    } else {
        None
    }
}

fn main() {
    let ResultValue: Result<i32 , &str>  = GetNumber(false).ok_or("Error value not found");
    println!("{:?}", ResultValue);
}
