fn main() {
    let v = vec![1,2,3];
    let v2 = v;
    display(v2);
    println!("In main {:?}", v2);
}

fn display (v:Vec<i32>) {
    println!("Inside display {:?}", v);
}

