fn FindMaximumSumRectangle(matrix: &[Vec<i32>]) -> i32 {
    let rows = matrix.len();
    let cols = matrix[0].len();
    let mut max_sum = i32::MIN;

    for left in 0..cols {
        let mut temp = vec![0; rows];

        for right in left..cols {
            for i in 0..rows {
                temp[i] += matrix[i][right];
            }

            let mut current_sum = 0;
            let mut best_sum = i32::MIN;

            for &num in &temp {
                current_sum = current_sum.max(0) + num;
                best_sum = best_sum.max(current_sum);
            }

            max_sum = max_sum.max(best_sum);
        }
    }

    max_sum
}

fn main() {
    let matrix = vec![
        vec![1, 2, -1, -4, -20],
        vec![-8, -3, 4, 2, 1],
        vec![3, 8, 10, 1, 3],
        vec![-4, -1, 1, 7, -6],
    ];
    println!("Maximum Sum Rectangle: {}", FindMaximumSumRectangle(&matrix));
}
