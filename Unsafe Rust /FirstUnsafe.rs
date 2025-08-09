fn bad_ref() -> &String {
let s = String::from("Hello");
&s
}

fn main() {
    let r = bad_ref();
    println!("{r}");
}

