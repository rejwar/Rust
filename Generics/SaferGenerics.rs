fn Largest<T: PartialOrd>(list: &[T]) -> Option<&T> {
    if list.is_empty() {
        return None;
    }

    let mut largest = &list[0];
    for item in list {
        if item > largest {
            largest = item;
        }
    }

    Some(largest)
}
