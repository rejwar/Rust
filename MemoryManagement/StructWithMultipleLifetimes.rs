struct Pair<'a, 'b> {
    first: &'a str,
    second: &'b str,
}

fn main() {
    let x = "First";
    let y = "Second";
    let pair = Pair { first: x, second: y };
    println!("{} and {}", pair.first, pair.second);
}
