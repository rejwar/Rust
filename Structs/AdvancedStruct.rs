

struct  Rectangle {
    Width: f64,
    Height: f64,
}

impl Rectangle {
    fn new(width: f64 , height: f64) -> Self {
        Rectangle { Width: width, Height: height }
    }

    fn area(&self) -> f64 {
        self.Width * self.Height
    }

    fn scale (&mut self , factor: f64) {
        self.Width *= factor;
        self.Height *= factor;
    }
}

struct  Container<T> {
    Value: T,
}

struct  BorrowedContnet <'a> {
    Text: &'a String,
}

fn main() {
    let mut rect = Rectangle::new(10.0 , 20.0);
    println!("Area : {}" , rect.area());
    rect.scale(2.0);

    let int_container = Container {Value: 42};
    let String_container = Container { Value: String::from("Hello")};

    let text = String::from("Hello");
    let borrow = BorrowedContnet {Text: &text};
}
