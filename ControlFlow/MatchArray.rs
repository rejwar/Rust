fn ArrayMatchExample(arr: &[i32]) {
    match arr {
        [1,2,3] => println!("Exactly [1,2,3,4]"),
        [1,..] => println!("Starts with 1 "),
        [..,3 ] => println!("Ends with 3"),
        [a ,b,c ]  => println!("Three elements {} ,{}, {}", a,b,c),
        _=> println!("Different array"),
    }
}

fn main() {
    ArrayMatchExample(&[1,2,3]);
    ArrayMatchExample(&[1,5,6]);
    ArrayMatchExample(&[4,5,3]);
    ArrayMatchExample(&[7,8]);
}
