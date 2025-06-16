fn main() {
    let rect1 = Rectangle { width: 30 , height: 30};
    let rect2 = Rectangle { width: 10 , height: 40};
    let rect3 = Rectangle { width: 40 , height: 60};

    println!(" Can rect1 hold rect2 {}" , rect1.can_hold(&rect2));

    println!(" Can rect1 hold rect2 {}" , rect2.can_hold(&rect3));

    println!(" Can rect1 hold rect2 {}" , rect1.can_hold(&rect2));

}
