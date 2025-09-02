fn good_ref() -> String {
    let s = String::from("hello");
    s
}

fn main() {
    let r = good_ref();
    println!("{r}");
}

