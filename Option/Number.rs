fn main() {
    let Number = Some(42);
    let EvenNumber = Number.filter(|&x| x % 2 ==0);

    println!("Number: {:?}", EvenNumber);
    

}
