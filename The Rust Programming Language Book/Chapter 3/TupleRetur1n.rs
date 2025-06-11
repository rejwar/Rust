fn compute(x:i32 , y:i32) ->(i32, i32 , i32 ) {
    (x+y , x-y , x*y)
}

fn main(){
    let (sum , diff, prod) = compute(10, 5);
    println!(" Sum {} , diff {} , prod {}", sum , diff , prod);
}
