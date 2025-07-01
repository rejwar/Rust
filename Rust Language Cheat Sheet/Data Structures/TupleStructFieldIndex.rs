struct Wrapper (i32 );

fn main() {
    let x =42;

    let a = Wrapper(x);

    let b = Wrapper{ 0 :x};

    println!(" a.0 = {} , b.0 ={}", a.0, b.0);
}
