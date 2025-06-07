fn main() {
    let vec1 = vec![1, 2, 3];
    let vec2 = vec![4, 5, 6];
    let vec3 = vec![7, 8, 9];

    println!("Vector 1: {:?}", vec1);
    println!("Vector 2: {:?}", vec2);
    println!("Vector 3: {:?}", vec3);

    println!("Vec == vec2: {}", vec1.eq(&vec2));
    println!("Vec == vec3: {}", vec1.eq(&vec3));
    println!("Vec == vec1: {}", vec1.eq(&vec1));
}
