fn SafeDivide(x:i32 , y:i32) ->Option<i32> {
    if y == 0 {
        None
    } else {
        Some(x/y)
    }
}

fn main() {
    println!("{:?}", SafeDivide(10, 2));
    println!("{:?}", SafeDivide(10, 0));
}
