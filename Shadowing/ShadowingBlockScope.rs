fn main() {
    let x:i32 = 5;
    {
        let x =12;
        assert_eq!( x, 12);
    }
    assert_eq!(x, 5);

    let x = 42;
    println!("{}",x);
}
