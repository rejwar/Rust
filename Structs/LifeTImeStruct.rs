#[derive(Debug)]
struct SliceView<'a> {
    title: &'a str,
    data: &'a [i32],
}

fn main() {
    let nums = vec![1,2,3,5];
    let view = SliceView {title : "First Two " , data: &nums [1..2]};
    println!("{:?}" , view);
}