fn main() {
    let v = 1_024 + 0xff + 0b111 + 0b1111_1111;
    assert!(v ==1579);

    println!("The value of v is: {}", v);
    

}
