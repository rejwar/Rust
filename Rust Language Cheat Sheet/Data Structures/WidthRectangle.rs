struct  Rectangle {
    width: i32 ,
    height: i32,
}

fn main() {
    let rect = Rectangle {
        width: 30,
        height: 40,
    };

    println!( "  width :{} , height: {} , ", rect.width, rect.height);
}
