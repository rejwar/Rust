struct Rectangle<T> {
    width: T,
    Height: T,
}

impl <T> Rectangle<T> {
    fn area (&self) {
        println!("Width : {:?} , Height : {:?}", self.width , self.Height);
    }
}

fn main() {
    let rect = Rectangle { width: 10 , Height: 5};
    rect.area();
}