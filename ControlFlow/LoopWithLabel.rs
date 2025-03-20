fn main() {
    'outer: for i in 1..5 {
        for j in 1..5 {
            if j == 3 {
                break 'outer;
            }
            println!("i: {}, j: {}", i, j);
        }
    }
}
