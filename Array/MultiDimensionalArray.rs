fn main() {
    // 2D array
    let matrix: [[i32; 3]; 2] = [
        [1, 2, 3],
        [4, 5, 6]
    ];

    println!("2D Array: {:?}", matrix);
    println!("Element at [1][2]: {}", matrix[1][2]);

    println!("\nMatrix elements:");
    for row in &matrix {
        for element in row {
            print!("{} ", element);  // Added space after each element
        }
        println!();
    }

    // 3D array (fixed syntax)
    let cube: [[[i32; 2]; 2]; 2] = [
        [
            [1, 2],  // Added comma
            [3, 4]   // Added comma
        ],
        [
            [5, 6],  // Added comma
            [7, 8]   // Added comma
        ]
    ];

    println!("\n3D Array: {:?}", cube);
    println!("Element at [1][0][1]: {}", cube[1][0][1]);
}
