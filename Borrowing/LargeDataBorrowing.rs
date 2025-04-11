fn ProcessData(Data: &Vec<i32>) {
    for Number in Data {
        println!("{}", Number);
    }
}

fn main() {
    let LargeData: Vec<i32> = (1..=10).collect();
    ProcessData(&LargeData);
}
