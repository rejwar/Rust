fn main() {
    let x =3;

    match x {
        ..=5 => println!("x is less or equal to 5"),
        _=> println!("x is greater than 5"),
    }
}
