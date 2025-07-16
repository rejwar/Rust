fn main() {
    let array = [1,2,3];
    println!("{}", array[0]);

    let mut array2: [i32; 3] = [4,5,6];
    println!("{}", array[0]);
    array2[0] =10;
    println!("{}", array2[3]);
}