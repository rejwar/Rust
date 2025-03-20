fn main() {
    let mut data = vec![1, 2, 3];
    modify(&mut data);
    println!("{:?}", data);
}

fn modify(values: &mut Vec<i32>) {
    values.push(4);
}
