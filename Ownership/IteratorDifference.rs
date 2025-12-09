fn main() {
    let numbers = vec![10, 20, 30];

    println!(" using iter () :");

    for num in numbers.iter() {
        println!(" Value : {}", num);
    }

    println!("Vector is still here {:?}", numbers);

    println!("\n using into iter ()");

    for num in numbers.into_iter() {
        println!(" Value is {}", num);
    }
}
