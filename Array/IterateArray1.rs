fn PrintElements(TheArray: &[i32]) {
    println!(" __Array Elements__");
    for Element in TheArray.iter() {
        println!("Element : {}", Element);
    }
    println!("___________");
}

fn main() {
    let MyData: [i32; 5] = [5,10,15,20,25];
    println!("________");

    for Index in 0..MyData.len() {
        println!("Index {} : {}", Index, MyData[Index]);
    }

    println!("____________");
}
