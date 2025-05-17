fn main() {
    let v = vec![1,2,3];
    let v2 = v;
    let v2_return = display(v2);
    println!("In main {:?}", v2_return);
}

fn display (v:Vec<i32>) -> Vec<i32> {
    println!("Inside display {:?}", v);
}
