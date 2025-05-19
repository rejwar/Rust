fn ReverseArrayPoint(arr: [i32;4]) {
    for i in(0..arr.len()).rev(){
        println!("{}", arr[i]);
    }
}

fn main() {
    let values= [ 1,2,3,4];
    println!("Reversed Array :");
    ReverseArrayPoint(values);
}
