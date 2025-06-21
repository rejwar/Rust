fn main() {
    'outer: for i in 1..=10 {
        for j in 1..=10 {
            if i ==2 && j ==2 {
                println!("Breaking outer loop is {} and {}", i,j);
                break 'outer;
            }
            println!("i ={} , j={}", i,j);
        }
    }
    println!("Existed outer loop");
}
