

fn main() {
    let (sum , product) = math_ops(3 ,4);
    println!("sum = {}, Product = {}" , sum, product );
}

fn math_ops(a: i32 , b: i32 ) -> (i32 , i32 ) {
    (a+b , a*b)
}