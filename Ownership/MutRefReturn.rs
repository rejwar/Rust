fn main() {
    let mut num = 10;
    let r = get_mut(&mut num);
    *r += 20;

    println!("Updated {}", num);
}

fn get_mut(n: &mut i32) -> &mut i32 {
    n
}
