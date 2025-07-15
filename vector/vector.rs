fn main() {
    let mut Number = vec![1,2,3,4,5,6];

    println!("First : {}", Number[0]);
    println!("Second : {}", Number[1]);

    Number.push(40);
    println!("After push : {:?}", Number);

    Number.pop();
    println!("After pop: {:?}", Number);

    for value in &Number {
        println!("Value : {}", value);
    }

    match Number.get(2) {
        Some(value) => println!("Third element {}", value),
        None => println!("No third element found"),
    }
}