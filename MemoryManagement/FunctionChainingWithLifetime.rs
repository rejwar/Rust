fn main() {
    let data = String::from("Data");
    println!("{}", step2(step1(&data)));
}

fn step1<'a>(input: &'a str) -> &'a str {
    input
}

fn step2<'a>(input: &'a str) -> &'a str {
    input
}
