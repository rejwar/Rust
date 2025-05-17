fn main() {
    let v = vec![10,20,30];
    print_vector (v);
    println!("{}",v[0]);
}

fn print_vector(x:Vec<i32>) {
    println!("Inside print_vector function {:?}", x);
}
