fn swap< A ,B>(a: A , b: B) -> (B,A) {
    (b,a)
}

fn main() {
    let result = swap("Hello", 42);
    println!("{:?}", result);

}
