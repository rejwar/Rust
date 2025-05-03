mod utils; 
mod math  {
    pub mod calc;
}

fn main() {
    crate::utils::PrintMessage();
    let result = crate::math::calc::Add(3,4);
    println!("Sum {}" , result);
}
