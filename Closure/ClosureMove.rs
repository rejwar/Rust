fn main() {
    let Data = vec![1,2,3];
    let print  = move || println!("Moved data {:?}", Data);

    print();
}