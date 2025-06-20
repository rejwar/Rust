fn main() {
    let make_x_odd = true;

    let x = if make_x_odd {
        1
    } else {
        2
    };
    println!("x is {}", x);
    let y = if make_x_odd {
        3
    } else {
        4
    };
    println!("y is {}", y);
}
