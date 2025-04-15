fn  MakeAdder (x: i32 ) -> impl Fn(i32) -> i32 {
    move |y| x+y
}

fn main() {
    let add_five = MakeAdder(5);
    println!("Result :{}" , add_five(10));
}
