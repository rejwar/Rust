struct  Counter {
    count: i32,
}

fn IncreamentCounter(c: &mut Counter) {
    c.count +=1;
}

fn main() {
    let mut my_counter = Counter{ count:0};
    IncreamentCounter(&mut my_counter);
    println!(" Updated Count: {}", my_counter.count);
}
