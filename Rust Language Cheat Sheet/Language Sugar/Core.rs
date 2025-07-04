use core::num;

fn main() {
    let name = String::from("Md");
    let numbers = vec![1,2,3];

    println!("Hello {}!", name);
    println!("Sum {}", numbers.iter().sum::<i32>());
}

