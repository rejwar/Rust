fn main() {
    let t = (String::from("Keep this "), 42);
    let s = take_String(t);

    println!("Git string {}", s);
}

fn take_String(data: (String, i32)) -> String {
    data.0
}
