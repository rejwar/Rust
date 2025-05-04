use std::default;

fn ResultChaining() {
    let res: Result<i32 , &str> = Ok(5);


    let mapped = res.map(|v| v * 2);
    let chained = mapped.and_then(|v| if v > 8 {Ok(v)} else {Err("Too small")});

    let default = chained.unwrap_or(999);
    println!("{}" , default);
}
