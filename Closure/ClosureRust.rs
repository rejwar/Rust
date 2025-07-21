
    fn create_multiplier(factor: i32 ) -> impl Fn(i32 ) -> i32  {
        move |x : i32 | -> i32 {x * factor}
    }

fn main() {
    let multiply_by_3 = create_multiplier(3);

    let result = multiply_by_3(10);
    println!("Result {}", result);

}   
