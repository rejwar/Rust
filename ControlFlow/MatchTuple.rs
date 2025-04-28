fn TupleMatchExample(coord: (i32 , i32)) {
    match coord {
        (0 ,0) => println!("At the Origin"),
        (x , 0) => println!("On x-axis at {}" , x),
        (0, y) => println!("On y-axis at {}" , y),
        (x ,y ) => println!("At ({} , {})", x, y),
    }
}

fn main() {
    TupleMatchExample((0,0));
    TupleMatchExample((5 ,0));
    TupleMatchExample((0,7));
    TupleMatchExample((3,4));
}
