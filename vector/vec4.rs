use std::io::Split;

fn main() {
    let mut Data =vec![1,2,3,4,5,6,7,8];

    let Drained: Vec<_> = Data.drain(2..4).collect();
    println!("Drained {:?}", Drained);
    println!("Remaining {:?}", Data);

    let Split = Data.split_off(2);
    println!("Split part : {:?}", Split);
    println!("Original after split {:?}", Data);
}