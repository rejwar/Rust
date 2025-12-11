fn main() {
    let arr = [String::from("A"), String::from("B")];

    let [a, b] = arr;

    println!("Owned {} {}", a, b);
}
