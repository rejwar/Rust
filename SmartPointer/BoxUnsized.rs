fn UseBoxUnsized() {
    let s = String::from("Hello Box");
    let BoxEd_Str: Box<str> = s.into_boxed_str();
    let arr = vec![1, 2, 3, 4, 5];
    let Boxed_slice: Box<[i32]> = arr.into_boxed_slice();
    println!(" Boxed Str {}", BoxEd_Str);
    println!(" Boxed slice length {}", Boxed_slice.len());
}

fn main() {
    UseBoxUnsized();
}
