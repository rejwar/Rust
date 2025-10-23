fn main() {
    let x = -3.7_f64;
    println!("abs {}", x.abs());
    println!("floor {} ceil {} round {}", x.floor(), x.ceil(), x.round());

    let a = 12;
    let b: i32 = 20;

    println!("mid {} max {}", a.min(b), a.max(b));
    println!("clamp {}", 150.clamp(0, 100));
}
