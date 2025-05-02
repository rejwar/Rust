#[derive(Debug)]

enum MyOption {
    Some(i32),
    None,
}



impl MyOption {
    fn unwrap(self) ->i32{
        match  self {
            MyOption::Some(value) =>value,
            MyOption::None =>panic!()
            
        }
    }
}

fn main() {}
