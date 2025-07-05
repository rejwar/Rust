fn never_returns() -> ! {
    panic! ("Never returns ");
}

fn get_values() -> i32 {
    if true {
        never_returns()
    } else {
        43a
    }
}
