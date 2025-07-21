fn is_positive(x: &i32 ) -> bool {
    *x > 0
}

fn main() {
    let numbers = vec![-5, -3, 0 , 2,7,-1];

    let positives: Vec<_> = numbers.into_iter().filter(is_positive).collect();
    println!("positive numbers {:?}", positives);
}

