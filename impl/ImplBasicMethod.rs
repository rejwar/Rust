struct Point {
    x: i32 ,
    y: i32,
}

impl Point {
    fn Display(&self) {
        println!("{} , {}", self.x , self.y);
    }
}