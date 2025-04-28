fn CreateBlock() -> i32 {
    let block = {
        let a = 4;
        let b = 6;
        a*b
    };

    block
}

fn main() {
    println!("Block Result {}", CreateBlock());
}
