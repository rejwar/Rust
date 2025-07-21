fn Apply<F: Fn(i32 ) -> i32> (f: F) {
println!("Applied {}", f(7));
}

fn main() {
    let Square = |x| x *x;
    Apply(Square);
}