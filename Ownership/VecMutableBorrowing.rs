fn ElementAdded(v: &mut Vec<i32>) {
    v.push(60);
    println!(" New element added {:?}", v);
}

fn main() {
    let mut v1 = vec![10, 20, 30, 40];
    ElementAdded(&mut v1);
    println!("Elements {:?}", v1);
}
