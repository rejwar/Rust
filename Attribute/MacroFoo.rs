#![allow(unused)]
fn main() {
/// This is different from [`foo!()`].
fn foo() {}

/// This is different from [`foo()`]
macro_rules! foo {
  () => {}
}
}
