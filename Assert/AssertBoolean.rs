fn main() {
    let f: bool = false;
    let g: bool = true && false;

    assert!(!f, "f should be false");
    assert!(!g, "g should be false");
    assert_eq!(g , f);

    println!("Assertions passed successfully!");
}
