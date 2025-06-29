struct S;

trait MyTrait {
    fn Do_something();
}

impl MyTrait for  S{
    fn Do_something() {
        println!("S does something");
    }
}

fn main() {
    S::Do_something();
}
