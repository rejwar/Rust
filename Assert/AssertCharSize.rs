use std::mem::size_of_val;


fn main() {
    let c1: char = 'a';
    assert_eq!(size_of_val(&c1), 4, "Size of char should be 4 bytes");

    let c2: char = 'b';
    assert_eq!(size_of_val(&c2), 4, "Size of char should be 4 bytes");


    let c3: char = 'c';
    assert_eq!(size_of_val(&c3), 4, "Size of char should be 4 bytes");

    println!("All assertions passed successfully!");

}
