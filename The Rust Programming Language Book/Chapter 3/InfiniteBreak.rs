fn main() {
    let mut Count=0;

    while true {
        if Count >= 3 {
            break;
        }
        println!("Loop Count : {}", Count);
        Count +=1;
    }
    println!("Loop terminated Manually !");
}
