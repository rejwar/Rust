fn Check_positions(coord: (i32 , i32)) {
    match coord {
        (0 ,0) => println!("Origin"),
        (_ , 0) => println!("On x-axis"),
        (0, _) => println!("On y-axis"),
        (_,_) => println!("Elsewhere"),

    }
}

fn main() {
    Check_positions((6,8));
}
