fn calculate(x:i32 , y:i32) -> (i32, i32 , i32) {
    (x+y , x-y , x*y)
}

fn main(){
    let (sum , difference , Product) = calculate(5, 10);
    println!("sum :{} , Difference {} , Product {}", sum , difference , Product);
}
