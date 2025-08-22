fn main() {
    let Data:String= String::from("Immutable Value");
    BorrowImmutable(&Data);
    BorrowImmutable(&Data);
    println!("Owner Still Valid {}", Data);
}

fn BorrowImmutable (Input: &String) {
    println!("Read Only Access {}", Input);
}