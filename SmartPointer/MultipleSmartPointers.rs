fn main() {
    let BoxedValue = Box::new(5);
    println!("Boxed value {}", BoxedValue);

    let SharedValue = std::rc::Rc::new(String::from("Shared Value"));

    let MutableData = std::cell::RefCell::new(vec!([1,2,3]));

    let ThredSafeData = std::sync::Arc::new(42);
}
