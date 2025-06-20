fn main() {
    let outer = 10;

    {
        let inner = 20;
        println!("Inner scope: outer = {}, inner = {}", outer, inner);
    }
}
