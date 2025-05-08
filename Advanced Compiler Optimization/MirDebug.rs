#![feature(rustc_attrs)]

#[rustc_mir]
fn calculate() -> i32 {
    let x = 10;
    let y = 20;
    x + y
}
