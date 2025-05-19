fn OutOfBoundPanic() {
    let arr = [1,2,3];
    println!("{}", arr[5]);
}

fn main() {
    OutOfBoundPanic();
}
