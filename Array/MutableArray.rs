use std::sync::mpsc::channel;

fn ChangeArray() {
    let mut data = [0;5];
    data[2] = 100;

    println!("Modified array: {:?} " , data);
}

fn main()
{
    ChangeArray();
}
