fn main() {
    let String1 = String::from("ABC");
    let String2 = ("xyz");

    let Result = Longest(&String1 , String2);
    println!("The longest string is {}", Result);
}

fn Longest<'a> (x: &'a str , y:&'a str)-> &'a str {
    if x.len() > y.len() {x} else {y}
}