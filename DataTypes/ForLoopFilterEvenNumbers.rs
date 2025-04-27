fn main() {
    for I in (1..10).filter(|x| x % 2 ==0) {
        println!("Even Number : {}" , I);
    }
}
