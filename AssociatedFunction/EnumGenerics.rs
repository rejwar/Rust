enum MyOption<T> {
    Some(T),
    None,
}

fn main() {
    let a: MyOption<i32> = MyOption::Some(10);
    let b: MyOption<&str> = MyOption::None;

    match a {
        MyOption::Some(val) => println!("Value: {}", val),
        MyOption::None => println!("No value"),
    }
    match b {
        MyOption::Some(val) => println!("Value: {}", val),
        MyOption::None => println!("No value"),
    }
}
