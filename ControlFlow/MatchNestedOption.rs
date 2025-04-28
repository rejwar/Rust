fn NestedOptionMatch (opt : Option<Option<i32>>) {
    match opt  {
        Some( Some(value)) => println!("Go inner value : {}" , value),
        Some( None) => println!("Inner is None"),
        None => println!("Outer is None"),
    }
}

fn main() {
    NestedOptionMatch(Some(Some(5)));
    NestedOptionMatch(Some(None));
    NestedOptionMatch(None);
}
