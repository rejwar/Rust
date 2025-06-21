fn main() {
    'row: for i in 1..=3{
        for j in 1..=3 {
            if j ==2 {
                continue 'row;
            }
            println!("i {} , j {}", i,j);
        }
    }
}
