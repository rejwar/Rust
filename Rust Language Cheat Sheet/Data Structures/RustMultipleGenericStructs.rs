#[derive(Debug)]
struct S<T> {
    x: T,
}


fn main() {
    let int_structs = S { x : 42};
    let str_structs = S { x : "Rust"};
    let vec_struct = S { x : vec![1,2,3,4,5]};

    println!(" {:?}", int_structs);
    println!(" {:?}", str_structs);
    println!( " {:?}", vec_struct);
}
