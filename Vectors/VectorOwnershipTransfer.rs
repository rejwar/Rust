fn TransferOwneship(Numbers : Vec<i32>) {
    println!("Received Vector : {:?}" , Numbers);
}

fn main() {
    let Data: Vec<i32> = vec![10,20,30];
    TransferOwneship(Data);
}
