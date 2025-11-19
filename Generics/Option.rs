enum Option<T> {
    Some(T),
    None,
}

enum Result<T, E> {
    Ok(T),
    Err(E),
}

fn MaybeNumber(flag: bool) -> Option<i32> {
    if flag {
        Option::Some(42)
    } else {
        Option::None
    }

    fn TryOpen(flag: bool) -> Result<&'static str, &'static str> {
        if flag {
            Result::Ok("Opended ")
        } else {
            Result::Err("Error")
        }
    }
}
