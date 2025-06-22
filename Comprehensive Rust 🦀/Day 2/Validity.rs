fn main() {
    let x_ref = {
        let x = 10;
        &x
    };
    dbg!(x_ref);
}
