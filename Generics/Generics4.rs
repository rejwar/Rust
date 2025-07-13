enum Result<T , E> {
    Ok(T),
    Err(E),
}

fn main() {
    
    let success: Result<i32 , &str> = Result::Ok(400) ;
    let error: Result<&str , i32 > = Result::Err(100);
}