fn repeat<const N: usize> (x: i32 ) -> [i32; N] {
    [x; N]
}

fn main() {
    let arr = repeat::<4>(7);
    println!("{:?}", arr);
}
