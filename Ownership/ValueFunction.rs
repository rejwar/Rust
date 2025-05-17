use std::path::Display;

fn main() {
    let v = vec![1,2,4];
    let v2 = v;
    Display(v2);
    println!("In main {:?}", v2);
}

fn Display (v:Vec<i32>) {
    println!("Inside display {:?}", v);
}
