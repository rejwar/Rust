struct Counter {
    Value: i32,
}

fn Increment(Counter : &mut Counter) {
    Counter.Value +=1;
}

fn main() {
    let mut MyCounter: Counter = Counter { Value: 0 };
    Increment(&mut MyCounter);
    println!("updated Counter : {}" , MyCounter.Value);
}
