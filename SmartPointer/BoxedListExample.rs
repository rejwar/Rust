fn main() {
  let b = Box::new(5);

  println!("Value in box : {}", b);

  #[derive(Debug)]

  enum List {
    Cons (i32 , Box<List>),
    Nil,
  }

  use List::{Cons , Nil};

  let List = Cons(1,  Box::new(Cons(3, Box::new(Nil))) );
}
