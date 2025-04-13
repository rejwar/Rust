fn transform_tuple_data(data: &[(i32, i32)]) -> Vec<(i32, i32)> {
    data.iter()
        .map(|(x, y)| (x + 1, y * 2))
        .collect()
}

fn main() {
    let input = [(1, 2), (3, 4), (5, 6)];
    let result = transform_tuple_data(&input);

    for (x, y) in result {
        println!("x:{}, y:{}", x, y);
    }
}
