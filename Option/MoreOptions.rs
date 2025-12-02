fn find_even(x: i32) -> Option<i32> {
    if x % 2 == 0 {
        Some(x)
    } else {
        None
    }
}
