fn main() {
    let mut num = 100;
    let r = get_mut_ref(&mut num);
    *r += 50;
    println!("Num {}", num);
}

fn get_mut_ref(n: &mut i32) -> &mut i32 {
    n
}
