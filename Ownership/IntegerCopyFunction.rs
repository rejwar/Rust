fn MakesCopy(SomeInteger: i32) {
    println!("{} Copied " ,SomeInteger);
}

fn main() {
    let x = 5;
    MakesCopy(x);
    println!("X is still {}",x);
}
