fn main() {
    let Numbers:Vec<i32> = vec![1,2,3,4,5,6];
    let EvenNumbers:Vec<i32> = Numbers.iter().filter(|&&x| x % 2 == 0).cloned().collect();

    println!("Even Numbers {:?}", EvenNumbers);ca
} 
