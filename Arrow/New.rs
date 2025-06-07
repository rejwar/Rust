fn main() {
    let repeated = [1,2].repeat(3);
    println!("{:?}", repeated);
    // Output: [1, 2, 1, 2, 1, 2]

    let mut vec = Vec::new();
    vec.extend(repeated);
    println!("{:?}", vec);
    vec.push(3);
    println!("{:?}", vec);

    println!("{:?}", vec);
    // Output: [1, 2, 1, 2, 1, 2]
}
