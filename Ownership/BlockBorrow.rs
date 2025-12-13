fn main() {
    let mut x = 100;

    {
        let r = &mut x;
        *r += 50;

        println!(" Inside {}", r);
    }

    println!("Outside block {}", x);
}
