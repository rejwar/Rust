use std::fmt::Display;

struct Wrapper <T> {
    value: T,
}

impl <T : Display> Wrapper<T> {
    fn Show(&self) 
{
    println!("value {}", self.value);
}
}