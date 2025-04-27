trait Area<T> {
    fn CalculateArea(&self) -> T;
}

struct Rectangle <T> {
    width: T,
    height: T,
}

impl<T> Area<T> for Rectangle<T>
where 
T:std::ops::Mul<Output = T> + Copy,
{
    fn CalculateArea(&self) -> T {
        
            self.width * self.height
        
    }
}

fn main() {
    let rect = Rectangle {
        width: 5,
        height: 10,
    };
    println!("Area : {}" , rect.CalculateArea());
}
