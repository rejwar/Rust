fn main() {
    let scores: [i32; 5] = [85, 92, 78, 65, 90];

    println!("Using index:");
    for index in 0..scores.len() {
        println!("Score {} : {}", index + 1, scores[index]);
    }

    println!("\nUsing Values:");

    for score in scores.iter() {
        println!("Score: {}", score);
    }

    println!("\nUsing enumerate:");
    for (index, score) in scores.iter().enumerate() {
        println!("Student {}: {}", index + 1, score);
    }
}
