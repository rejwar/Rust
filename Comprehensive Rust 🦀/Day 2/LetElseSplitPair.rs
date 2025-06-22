// ❓ Question: How do you use `let…else` to destructure a tuple of Options, else panic?

fn split_pair(s: &str) -> (u64, &str) {
    let mut parts = s.split(' ');
    let (Some(count_str), Some(item)) = (parts.next(), parts.next()) else {
        panic!("Expected two parts, got '{}'", s);
    };

    let Ok(count) = count_str.parse::<u64>() else {
        panic!("Invalid number: '{}'", count_str);
    };

    (count, item)
}

fn main() {
    let (n, item) = split_pair("3 chairs");
    println!("Count: {}, Item: '{}'", n, item);
}
