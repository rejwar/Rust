fn main() {
    let mut MyScores: [i32; 4] = [70,80,90,200];
    println!("Original score at index 1 :{} ", MyScores[1]);

    MyScores[1] = 88;
    println!("Updated score at index 1 : {}", MyScores[1]);
}
