struct Rectangle {
    Width: u32,
    Height: u32,
}

// `Rectangle`-এর জন্য একটি ইমপ্লিমেন্টেশন ব্লক
impl Rectangle {
    // এই মেথডটি `&self` ব্যবহার করে, কারণ এটি শুধু ডেটা পড়ে
    fn Area(&self) -> u32 {
        self.Width * self.Height
    }

    // এই মেথডটি `&mut self` ব্যবহার করে, কারণ এটি ডেটা পরিবর্তন করে
    fn SetWidth(&mut self, Width: u32) {
        self.Width = Width;
    }
}

fn MethodExample() {
    let mut Rect = Rectangle { Width: 30, Height: 50 };

    println!("The area of the rectangle is {}", Rect.Area()); // Area() কল করা হলো

    Rect.SetWidth(35); // SetWidth() কল করে Width পরিবর্তন করা হলো
    println!("The new width is {}", Rect.Width);
}

fn main() {
    MethodExample();
}
