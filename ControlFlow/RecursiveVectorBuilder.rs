fn BuildVector(n:i32 , vec: &mut Vec<i32>) {
    if n == 0 {
        return ;
    } else {
        vec.push(n);
        BuildVector(n-1, vec);
    }
}

fn main() {
    let mut numbers = Vec::new();
    BuildVector(5, &mut numbers);

    println!("Number : {:?}" ,numbers);
}
