fn AnagramSort(s1: &str, s2: &str) -> bool {
    if s1.len() != s2.len() {
        return false;
    }

    let mut a: Vec<char> = s1.chars().collect();
    let mut b: Vec<char> = s2.chars().collect();

    a.sort();
    b.sort();

    a == b
}

fn main() {
    let s1 = " RUST";
    let s2 = " TSUR";

    println!(" {} ", AnagramSort(s1, s2));
}
