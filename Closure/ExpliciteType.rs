fn main() {
    let devide : fn(i32 , i32 ) -> f64 = |x,y| (x as f64) / (y as f64);

    let result = devide (10,3);
    println!("Result is {:.2}", result );
}
