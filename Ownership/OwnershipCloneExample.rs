fn CloneOwnership() {
    let name1 = String::from("Rust");
    let name2 = name1.clone();

    println!("Name1: {} , Name2 {}" , name1,name2);
}

fn main() {
    CloneOwnership();
}
