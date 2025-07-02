fn main() {
    let arr = [10,20,30,40,50];
    let a =2; 

    let slice = &arr[a..];
    println!("{:?}", slice);
}
