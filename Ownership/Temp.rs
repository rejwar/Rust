fn ReturningDangling()-> &String {
    let Temp: String = String::from("Dangling");
    &Temp
}

fn main() {
    let Ref: &String = ReturningDangling();
    println!("{}", Ref);
}