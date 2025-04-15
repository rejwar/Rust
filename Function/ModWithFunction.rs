use std::result;

mod math {
    pub fn Multiply (a: i32 , b:i32) -> i32 {
        a*b
    }

}

fn main() {
    let result = math::Multiply(6, 7);
    println!("Product : {}" ,result);
}
